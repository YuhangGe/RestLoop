use std::sync::{
  Arc,
  atomic::{AtomicBool, AtomicU8, AtomicU32},
  mpsc::Receiver,
};

use display_info::DisplayInfo;
use eframe::{
  CreationContext,
  egui::{
    self, Color32, Frame, IconData, Id, Pos2, RichText, Sense, Vec2, ViewportBuilder,
    ViewportCommand,
  },
};

#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::counter::CounterMessage;

const COUNTER_WINDOW_WIDTH: f32 = 138f32;
const COUNTER_WINDOW_HEIGHT: f32 = 70f32;
const COUNTER_WINDOW_RADIUS: f32 = 24f32;
const COUNTER_WINDOW_FONT_SIZE: f32 = 40f32;

struct CounterApp {
  scale: f32,
  close_signal: Arc<AtomicBool>,
  count_secs: Arc<AtomicU32>,
}

impl eframe::App for CounterApp {
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
  }
  fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
    let panel_frame = Frame::default()
      .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60))
      .corner_radius(COUNTER_WINDOW_RADIUS / self.scale);

    egui::CentralPanel::default()
      .frame(panel_frame)
      .show(ctx, |ui| {
        ui.style_mut().interaction.selectable_labels = false;
        let res = ui.interact(ui.max_rect(), Id::new(1), Sense::click_and_drag());
        if res.dragged() {
          // ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        } else if res.clicked() {
          ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(900f32, 900f32)));
          ctx.send_viewport_cmd(ViewportCommand::OuterPosition(Pos2::new(0f32, 0f32)));
        }
        ui.centered_and_justified(|ui| {
          ui.label(
            RichText::new("00:00")
              .monospace()
              .size(COUNTER_WINDOW_FONT_SIZE / self.scale)
              .color(Color32::WHITE),
          );
        });
      });
  }
}
pub fn show_counter_window(count_secs: u32, rx: Receiver<CounterMessage>) {
  let displays = DisplayInfo::all().unwrap();
  let primary_display = displays.iter().find(|d| d.is_primary).unwrap();
  let scale = primary_display.scale_factor;
  let screen_width = (primary_display.width as f32) / scale;
  let width = COUNTER_WINDOW_WIDTH / scale;
  let height = COUNTER_WINDOW_HEIGHT / scale;
  let top = 100f32 / scale;
  let right = 100f32 / scale;
  let pos = (screen_width - width - right, top);
  // println!("POS: {:#?} {}", pos, primary_display.scale_factor);
  let options = eframe::NativeOptions {
    event_loop_builder: Some(Box::new(|_elb| {
      #[cfg(windows)]
      _elb.with_any_thread(true);
    })),
    viewport: ViewportBuilder::default()
      .with_icon(IconData::default())
      .with_inner_size((width, height))
      .with_always_on_top()
      .with_transparent(true)
      .with_drag_and_drop(true)
      .with_decorations(false)
      .with_position(pos)
      .with_taskbar(false),
    ..Default::default()
  };

  let close_signal = Arc::new(AtomicBool::new(false));
  let count_secs = Arc::new(AtomicU32::new(count_secs));
  let counter_app = Box::new(CounterApp {
    scale,
    close_signal: close_signal.clone(),
    count_secs: count_secs.clone(),
  });
  std::thread::spawn(move || {
    for msg in rx {
      match msg {
        CounterMessage::Close => {
          close_signal.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        CounterMessage::Update(v) => {
          count_secs.store(v, std::sync::atomic::Ordering::Relaxed);
        }
      }
    }
  });
  eframe::run_native("工作计时器", options, Box::new(move |_| Ok(counter_app))).unwrap();
}
