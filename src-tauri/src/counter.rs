use std::{
  sync::{
    self, Arc, Mutex,
    atomic::{AtomicBool, AtomicU32},
    mpsc::Sender,
  },
  time::Duration,
};

use tokio_util::sync::CancellationToken;

use crate::{message::Message, window_counter::show_counter_window};

pub type CounterState = Arc<Mutex<Counter>>;

pub struct Counter {
  count_secs: Arc<AtomicU32>,
  work_secs: u32,
  canncel_token: Option<CancellationToken>,
  sx: Option<Arc<Sender<Message>>>,
  paused: Arc<AtomicBool>,
}

impl Counter {
  pub fn new(work_secs: u32) -> Self {
    Self {
      count_secs: Arc::new(AtomicU32::new(0)),
      work_secs,
      canncel_token: None,
      sx: None,
      paused: Arc::new(AtomicBool::new(false)),
    }
  }
  pub fn new_state(work_secs: u32) -> CounterState {
    Arc::new(Mutex::new(Counter::new(5)))
  }
  pub fn is_paused(&self) -> bool {
    self.paused.load(sync::atomic::Ordering::Relaxed)
  }
  pub fn set_work_secs(&mut self, work_secs: u32) {
    self.work_secs = work_secs;
    // self.start();
  }

  pub fn start(&mut self, app_sx: Arc<Sender<Message>>) {
    self.close(); // close previous if need

    self.count_secs.store(0, sync::atomic::Ordering::Relaxed);
    let ct = CancellationToken::new();
    self.canncel_token.replace(ct.clone());
    let count_secs = self.count_secs.clone();
    let work_secs = self.work_secs;

    let (sx, rx) = sync::mpsc::channel::<Message>();
    let sx = Arc::new(sx);
    self.sx.replace(sx.clone());

    let init_count_secs = count_secs.load(sync::atomic::Ordering::Relaxed);
    std::thread::spawn(move || {
      show_counter_window(init_count_secs, rx);
    });

    let is_paused = self.paused.clone();
    tauri::async_runtime::spawn(async move {
      loop {
        let ha = async {
          tokio::time::sleep(Duration::from_secs(1)).await;

          if is_paused.load(sync::atomic::Ordering::Relaxed) {
            println!("paused.");
            return true;
          }

          if ct.is_cancelled() {
            println!("cancelled.");
            return false;
          }

          let secs = count_secs.fetch_add(1, sync::atomic::Ordering::Relaxed) + 1;
          println!("counter tick {}, {}", secs, work_secs);
          sx.send(Message::Update(secs)).unwrap();
          if secs == work_secs {
            sx.send(Message::Close).unwrap();
            app_sx.send(Message::CounterEnd).unwrap();
            false
          } else {
            true
          }
        };
        let hb = ct.cancelled();
        tokio::select! {
          ret = ha => {
            if !ret {
              break;
            }
          },
          _ = hb => {
            break;
          }
        }
      }
    });
  }
  pub fn close(&mut self) {
    self.canncel_token.take().map(|ct| {
      ct.cancel();
    });
    self.sx.take().map(|tx| {
      tx.send(Message::Close).unwrap();
    });
  }
  pub fn reset(&mut self) {
    self.count_secs.store(0, sync::atomic::Ordering::Relaxed);
  }
  pub fn pause(&mut self) {
    self.paused.store(true, sync::atomic::Ordering::Relaxed);
  }

  pub fn resume(&mut self) {
    self.paused.store(false, sync::atomic::Ordering::Relaxed);
  }
}
