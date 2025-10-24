use std::{
  sync::{
    Arc, Mutex,
    atomic::{AtomicU32, Ordering::Relaxed},
    mpsc::{Sender, channel},
  },
  time::Duration,
};

use tokio_util::sync::CancellationToken;

use crate::{message::Message, window_blocker::show_blocker_window};

pub struct Blocker {
  rest_secs: u32,
  count_secs: Arc<AtomicU32>,
  canncel_token: Option<CancellationToken>,
  sx: Option<Arc<Sender<Message>>>,
}

pub type BlockerState = Arc<Mutex<Blocker>>;

impl Blocker {
  pub fn new(rest_secs: u32) -> Self {
    Self {
      rest_secs,
      count_secs: Arc::new(AtomicU32::new(0)),
      canncel_token: None,
      sx: None,
    }
  }
  pub fn new_state(rest_secs: u32) -> BlockerState {
    Arc::new(Mutex::new(Blocker::new(5)))
  }

  pub fn set_rest_secs(&mut self, rest_secs: u32) {
    self.rest_secs = rest_secs;
  }

  pub fn start(&mut self, app_sx: Arc<Sender<Message>>) {
    self.close(); // close previous if need

    let ct = CancellationToken::new();
    self.canncel_token.replace(ct.clone());
    let init_count_secs = self.rest_secs;
    let count_secs = self.count_secs.clone();
    count_secs.store(init_count_secs, Relaxed);

    let (sx, rx) = channel::<Message>();
    let sx = Arc::new(sx);
    self.sx.replace(sx.clone());

    let (sx2, rx2) = channel::<()>();

    std::thread::spawn(move || {
      show_blocker_window(init_count_secs, rx, sx2);
    });

    std::thread::spawn(move || {
      let _ = rx2.recv().unwrap();
      app_sx.send(Message::BlockerEnd).unwrap();
    });

    tauri::async_runtime::spawn(async move {
      loop {
        let ha = async {
          tokio::time::sleep(Duration::from_secs(1)).await;

          if ct.is_cancelled() {
            return false;
          }

          let secs = count_secs.fetch_sub(1, Relaxed) - 1;
          // println!("counter tick {}", secs);
          sx.send(Message::Update(secs)).unwrap();
          if secs <= 0 { false } else { true }
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
    self.sx.take().map(|sx| {
      sx.send(Message::Close).unwrap();
    });
  }
}
