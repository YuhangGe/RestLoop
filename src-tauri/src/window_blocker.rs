use std::sync::{
  Arc,
  atomic::{AtomicBool, AtomicU32},
  mpsc::{self, Receiver, Sender},
};

use display_info::DisplayInfo;
use eframe::egui::{
  self, Color32, Frame, IconData, Id, RichText, Sense, ViewportBuilder, ViewportCommand,
};

#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::{message::Message, util::pad};

struct BlockerApp {
  scale: f32,
  close_signal: Arc<AtomicBool>,
  mv_sx: Arc<Sender<bool>>,
  count_secs: Arc<AtomicU32>,
}

impl eframe::App for BlockerApp {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
  }
  fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
    if self.close_signal.load(std::sync::atomic::Ordering::Relaxed) {
      ctx.send_viewport_cmd(ViewportCommand::Close);
      return;
    }

    let count_secs = self.count_secs.load(std::sync::atomic::Ordering::Relaxed);

    let display = if count_secs <= 0 {
      "休息结束，滑动鼠标解锁".to_string()
    } else {
      let minutes = pad(count_secs / 60);
      let seconds = pad(count_secs % 60);
      format!("正在休息中，{}:{} 后解锁", minutes, seconds)
    };
    let panel_frame = Frame::default().fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 120));
    let mv_sx = self.mv_sx.clone();

    egui::CentralPanel::default()
      .frame(panel_frame)
      .show(ctx, move |ui| {
        ui.style_mut().interaction.selectable_labels = false;
        let res = ui.interact(ui.max_rect(), Id::new(1), Sense::click());
        if res.clicked() {
          mv_sx.send(true).unwrap();
        }
        ui.centered_and_justified(|ui| {
          ui.label(
            RichText::new(display)
              .monospace()
              .size(32f32 / self.scale)
              .color(Color32::WHITE),
          );
        });
      });

    ctx.request_repaint();
  }
}
pub fn show_blocker_window(count_secs: u32, rx: Receiver<Message>, sx: Sender<()>) {
  let displays = DisplayInfo::all().unwrap();
  let primary_display = displays.iter().find(|d| d.is_primary).unwrap();
  let scale = primary_display.scale_factor;
  let screen_width = (primary_display.width as f32) / scale;
  let screen_height = (primary_display.height as f32) / scale;

  // println!("POS: {:#?} {}", pos, primary_display.scale_factor);
  let options = eframe::NativeOptions {
    event_loop_builder: Some(Box::new(|_elb| {
      #[cfg(windows)]
      _elb.with_any_thread(true);
    })),
    viewport: ViewportBuilder::default()
      .with_icon(IconData::default())
      .with_inner_size((screen_width, screen_height))
      .with_always_on_top()
      .with_transparent(true)
      .with_decorations(false)
      .with_position((0.0, 0.0))
      .with_taskbar(false),
    ..Default::default()
  };

  let close_signal = Arc::new(AtomicBool::new(false));
  let (mv_sx, mv_rx) = mpsc::channel();
  let count_secs = Arc::new(AtomicU32::new(count_secs));
  let blocker_app = Box::new(BlockerApp {
    scale,
    mv_sx: Arc::new(mv_sx),
    close_signal: close_signal.clone(),
    count_secs: count_secs.clone(),
  });
  std::thread::spawn(move || {
    for msg in rx {
      match msg {
        Message::Close => {
          close_signal.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        Message::Update(v) => {
          count_secs.store(v, std::sync::atomic::Ordering::Relaxed);
        }
        _ => (),
      }
    }
  });
  std::thread::spawn(move || {
    let _ = mv_rx.recv().unwrap();
    sx.send(()).unwrap();
  });
  eframe::run_native(
    "RestLoop - 休息熄屏器",
    options,
    Box::new(move |_| Ok(blocker_app)),
  )
  .unwrap();
}
