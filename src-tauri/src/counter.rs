use std::{
  sync::{self, Arc},
  time::Duration,
};

use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::window_counter::show_counter_window;

pub type CounterState = Arc<Mutex<Counter>>;

#[derive(Clone, Copy)]
pub enum CounterMessage {
  Close,
  Update(u32),
}

pub struct Counter {
  count_secs: Arc<Mutex<u32>>,
  work_secs: u32,
  canncel_token: Option<CancellationToken>,
}

impl Counter {
  pub fn new(work_secs: u32) -> Self {
    Self {
      count_secs: Arc::new(Mutex::new(0)),
      work_secs,
      canncel_token: None,
    }
  }
  pub fn new_state(work_secs: u32) -> CounterState {
    Arc::new(Mutex::new(Counter::new(work_secs)))
  }
  pub async fn set_work_secs(&mut self, work_secs: u32) {
    self.work_secs = work_secs;
    self.start().await;
  }

  pub async fn start(&mut self) {
    *self.count_secs.clone().lock().await = 0;

    self.pause().await; // stop if need
    self.resume().await;
  }
  pub async fn pause(&mut self) {
    if let Some(ct) = self.canncel_token.take() {
      ct.cancel();
    }
  }
  pub async fn resume(&mut self) {
    let ct = CancellationToken::new();
    self.canncel_token.replace(ct.clone());
    let count_secs = self.count_secs.clone();
    let work_secs = self.work_secs;

    let (sx, rx) = sync::mpsc::channel::<CounterMessage>();
    let init_count_secs = *count_secs.lock().await;
    std::thread::spawn(move || {
      show_counter_window(init_count_secs, rx);
    });
    tokio::spawn(async move {
      loop {
        let ha = async {
          tokio::time::sleep(Duration::from_secs(1)).await;

          if ct.is_cancelled() {
            return false;
          }
          let mut x = count_secs.lock().await;
          let x2 = *x + 1;
          *x = x2;
          sx.send(CounterMessage::Update(x2)).unwrap();
          if x2 == work_secs {
            todo!();
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
}
