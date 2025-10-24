use std::{sync::Arc, time::Instant};

use display_info::DisplayInfo;
use eframe::egui::{
  CentralPanel, Color32, Frame, IconData, Id, Pos2, RichText, Sense, Vec2, ViewportBuilder,
  ViewportCommand, ViewportId, Visuals,
};

#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::{chinese_font::setup_chinese_fonts, settings::Settings, test::find_window};

const COUNTER_WINDOW_WIDTH: f32 = 142f32;
const COUNTER_WINDOW_HEIGHT: f32 = 70f32;
const COUNTER_WINDOW_RADIUS: f32 = 24f32;
const COUNTER_WINDOW_FONT_SIZE: f32 = 40f32;

#[derive(Debug, Clone, Copy)]
enum State {
  Counting,
  Blocking,
  BlockEnd,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
  None,
  EnterBlocking,
  EnterCounting,
  EnterBlockEnd,
}

struct CounterApp {
  work_secs: u32,
  rest_secs: u32,
  count_start_time: Instant,
  state: State,
  display: CounterDisplay,
  second_display: Option<CounterDisplay2>,
  event: Event,
  show_second: bool,
}

struct CounterDisplay {
  scale: f32,
  /// width, height
  screen_size: (f32, f32),
  /// left, top, width, height
  counting_rect: ((f32, f32), (f32, f32)),
}

struct CounterDisplay2 {
  scale: f32,
  screen_rect: ((f32, f32), (f32, f32)),
}

impl CounterApp {
  pub fn new(settings: &Settings) -> Self {
    let displays = DisplayInfo::all().unwrap();
    let primary_display = displays.iter().find(|d| d.is_primary).unwrap();
    let scale = primary_display.scale_factor;
    let screen_width = (primary_display.width as f32) / scale;
    let screen_height = (primary_display.height as f32) / scale;

    let width = COUNTER_WINDOW_WIDTH / scale;
    let height = COUNTER_WINDOW_HEIGHT / scale;
    let bottom = 80f32 / scale;
    let right = 30f32 / scale;

    let second_display = displays.iter().find(|d| !d.is_primary).map(|d| {
      let scale = d.scale_factor;
      let screen_width = (d.width as f32) / scale;
      let screen_height = (d.height as f32) / scale;
      return CounterDisplay2 {
        scale,
        screen_rect: (
          (d.x as f32, d.y as f32),
          (screen_width - 1.0, screen_height - 1.0),
        ),
      };
    });

    Self {
      display: CounterDisplay {
        scale: scale,
        screen_size: (screen_width - 1.0, screen_height - 1.0),
        counting_rect: (
          (
            screen_width - width - right,
            screen_height - bottom - height,
          ),
          (width, height),
        ),
      },
      second_display,
      work_secs: 3, // settings.work_secs,
      rest_secs: 4, //settings.rest_secs,
      state: State::Counting,
      count_start_time: Instant::now(),
      event: Event::None,
      show_second: false,
    }
  }
  fn place_window(&self, ctx: &eframe::egui::Context, pos: Pos2, size: Vec2) {
    ctx.send_viewport_cmd(ViewportCommand::OuterPosition(pos));
    ctx.send_viewport_cmd(ViewportCommand::InnerSize(size));
    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(
      eframe::egui::WindowLevel::AlwaysOnTop,
    ));
  }
  fn handle_event(&mut self, ctx: &eframe::egui::Context) -> bool {
    let evt = self.event;
    self.event = Event::None;
    match evt {
      Event::None => false,
      Event::EnterBlocking => {
        self.place_window(ctx, (0f32, 0f32).into(), self.display.screen_size.into());
        self.state = State::Blocking;
        self.count_start_time = Instant::now();

        if let Some(_) = self.second_display.as_ref() {
          self.show_second = true;
        };

        true
      }
      Event::EnterCounting => {
        self.place_window(
          ctx,
          self.display.counting_rect.0.into(),
          self.display.counting_rect.1.into(),
        );

        self.state = State::Counting;
        self.count_start_time = Instant::now();
        true
      }
      Event::EnterBlockEnd => {
        self.place_window(ctx, (0f32, 0f32).into(), self.display.screen_size.into());
        self.state = State::BlockEnd;
        true
      }
    }
  }
}

impl eframe::App for CounterApp {
  fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
    [0.0, 0.0, 0.0, 0.0]
  }

  fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
    // 必须在函数最开始消费 event
    if self.handle_event(ctx) {
      ctx.request_repaint();
      return;
    }

    let is_counting_state = matches!(self.state, State::Counting);
    let mut panel_frame = Frame::default().fill(Color32::from_rgba_unmultiplied(
      0,
      0,
      0,
      if is_counting_state { 60 } else { 120 },
    ));
    if is_counting_state {
      panel_frame = panel_frame.corner_radius(COUNTER_WINDOW_RADIUS / self.display.scale);
    }
    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
      ui.style_mut().interaction.selectable_labels = false;

      let display = match self.state {
        State::Counting => {
          let res = ui.interact(ui.max_rect(), Id::new(1), Sense::drag());
          if res.dragged() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
          }
          let passed_secs = Instant::now()
            .duration_since(self.count_start_time)
            .as_secs() as u32;

          let minutes = passed_secs / 60;
          let seconds = passed_secs % 60;

          if passed_secs >= self.work_secs {
            self.event = Event::EnterBlocking;
          }

          format!("{:02}:{:02}", minutes, seconds)
        }
        State::Blocking => {
          let passed_secs = Instant::now()
            .duration_since(self.count_start_time)
            .as_secs() as u32;
          let left_secs = self.rest_secs - passed_secs;
          let minutes = left_secs / 60;
          let seconds = left_secs % 60;

          if left_secs <= 0 {
            self.event = Event::EnterBlockEnd;
          }

          format!("休息中，{:02}:{:02} 后解锁", minutes, seconds)
        }
        State::BlockEnd => {
          let res = ui.interact(ui.max_rect(), Id::new(2), Sense::click());
          if res.clicked() {
            self.event = Event::EnterCounting;
          }
          "休息结束，移动鼠标解锁".to_string()
        }
      };

      if matches!(self.state, State::Blocking | State::BlockEnd) {
        if let Some(sd) = self.second_display.as_ref() {
          let display = Arc::new(display.clone());
          ctx.show_viewport_immediate(
            ViewportId::from_hash_of("second"),
            ViewportBuilder::default()
              .with_title("RestLoop-Second-Blocker")
              .with_position(sd.screen_rect.0)
              .with_inner_size(sd.screen_rect.1)
              .with_transparent(true)
              .with_decorations(false),
            move |ctx, class| {
              CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                  ui.label(
                    RichText::new(display.to_string())
                      .monospace()
                      .size(COUNTER_WINDOW_FONT_SIZE / sd.scale)
                      .color(Color32::WHITE),
                  );
                });
              });
            },
          )
        }
      }
      ui.centered_and_justified(|ui| {
        ui.label(
          RichText::new(display)
            .monospace()
            .size(COUNTER_WINDOW_FONT_SIZE / self.display.scale)
            .color(Color32::WHITE),
        );
      });
    });

    ctx.request_repaint();
  }
}
pub fn start_counter_app(settings: &Settings) {
  let counter_app = Box::new(CounterApp::new(settings));

  std::thread::spawn(move || {
    let init_rect = counter_app.display.counting_rect;

    let options = eframe::NativeOptions {
      event_loop_builder: Some(Box::new(|_elb| {
        #[cfg(windows)]
        _elb.with_any_thread(true);
      })),

      viewport: ViewportBuilder::default()
        .with_icon(IconData::default())
        .with_inner_size(init_rect.1)
        .with_always_on_top()
        .with_transparent(true)
        .with_drag_and_drop(true)
        .with_decorations(false)
        .with_position(init_rect.0)
        .with_taskbar(false),
      ..Default::default()
    };

    eframe::run_native(
      "RestLoop - 工作计时器",
      options,
      Box::new(move |cc| {
        setup_chinese_fonts(&cc.egui_ctx).unwrap();
        Ok(counter_app)
      }),
    )
    .unwrap();
  });
}
