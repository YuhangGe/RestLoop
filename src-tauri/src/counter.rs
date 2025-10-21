use display_info::DisplayInfo;
use eframe::egui::{
  self, Color32, Frame, IconData, Id, Pos2, RichText, Sense, Vec2, ViewportBuilder, ViewportCommand,
};
use winit::platform::windows::EventLoopBuilderExtWindows;

const COUNTER_WINDOW_WIDTH: f32 = 138f32;
const COUNTER_WINDOW_HEIGHT: f32 = 70f32;
const COUNTER_WINDOW_RADIUS: f32 = 24f32;
const COUNTER_WINDOW_FONT_SIZE: f32 = 40f32;

#[derive(Default)]
struct CounterApp {
  scale: f32,
  full: bool,
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
pub fn show_counter_window() {
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
    event_loop_builder: Some(Box::new(|elb| {
      elb.with_any_thread(true);
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

  eframe::run_native(
    "工作计时器",
    options,
    Box::new(|_| Ok(Box::new(CounterApp { scale, full: false }))),
  )
  .unwrap();
}
