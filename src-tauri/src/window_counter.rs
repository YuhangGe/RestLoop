use std::{
  sync::{Arc, atomic::AtomicU8},
  time::{SystemTime, UNIX_EPOCH},
};

use display_info::DisplayInfo;
use eframe::egui::{
  CentralPanel, Color32, Frame, IconData, Id, Key, Pos2, RichText, Sense, Vec2, ViewportBuilder,
  ViewportCommand, ViewportId, Visuals,
};

#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows;

use crate::{chinese_font::setup_chinese_fonts, settings::Settings};

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

const EVENT_ENTER_BLOCKING: u8 = 1;
const EVENT_ENTER_COUNTING: u8 = 2;
const EVENT_BLOCKING_END: u8 = 3;
pub const EVENT_PAUSE_COUNTING: u8 = 4;
pub const EVENT_RESUME_COUNTING: u8 = 5;
pub const EVENT_RESET_COUNTING: u8 = 6;

pub type CounterEventSignal = Arc<AtomicU8>;

struct CounterApp {
  work_secs: u32,
  rest_secs: u32,
  escape_count: u32,
  count_start_time: u64,
  count_paused_time: Option<u64>,
  state: State,
  display: CounterDisplay,
  second_display: Option<CounterDisplay2>,
  event_signal: Arc<AtomicU8>,
  mouse_pos: (u32, u32),
  escape_pressed_count: u32,
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

fn fmt_count(minutes: u32, seconds: u32) -> String {
  format!("休息中，{:02}:{:02} 后解锁", minutes, seconds)
}

fn now() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

impl CounterApp {
  pub fn new(settings: &Settings, event_signal: Arc<AtomicU8>) -> Self {
    let displays = DisplayInfo::all().unwrap();
    let primary_display = displays.iter().find(|d| d.is_primary).unwrap();
    let scale = primary_display.scale_factor;
    let screen_width = (primary_display.width as f32) / scale;
    let screen_height = (primary_display.height as f32) / scale;

    let width = COUNTER_WINDOW_WIDTH / scale;
    let height = COUNTER_WINDOW_HEIGHT / scale;
    let bottom = 80f32 / scale;
    let right = 30f32 / scale;

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
      second_display: None,
      work_secs: settings.work_secs,
      rest_secs: settings.rest_secs,
      escape_count: settings.escape_count,
      state: State::Counting,
      count_start_time: now(),
      count_paused_time: None,
      event_signal,
      mouse_pos: (0, 0),
      escape_pressed_count: 0,
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
    let evt = self.event_signal.load(std::sync::atomic::Ordering::Relaxed);
    self
      .event_signal
      .store(0, std::sync::atomic::Ordering::Relaxed);
    match evt {
      EVENT_PAUSE_COUNTING => {
        self.count_paused_time = Some(now());
        false
      }
      EVENT_RESUME_COUNTING => {
        self.count_paused_time.take().map(|v| {
          let pasued_secs = now() - v;
          self.count_start_time += pasued_secs;
        });
        false
      }
      EVENT_RESET_COUNTING => {
        self.count_start_time = now();
        self.count_paused_time = None;
        false
      }
      EVENT_ENTER_BLOCKING => {
        self.place_window(ctx, (0f32, 0f32).into(), self.display.screen_size.into());
        self.state = State::Blocking;
        self.count_start_time = now();
        self.escape_pressed_count = 0;
        let second_display = DisplayInfo::all()
          .unwrap_or_default()
          .iter()
          .find(|d| !d.is_primary)
          .map(|d| {
            let scale = d.scale_factor;
            let screen_width = (d.width as f32) / scale;
            let screen_height = (d.height as f32) / scale;
            // println!("{}, {} {}", scale, screen_width, screen_height);
            return CounterDisplay2 {
              scale,
              screen_rect: (
                ((d.x as f32) / scale, (d.y as f32) / scale),
                (screen_width, screen_height),
              ),
            };
          });
        self.second_display = second_display;

        true
      }
      EVENT_ENTER_COUNTING => {
        self.second_display.take();
        self.place_window(
          ctx,
          self.display.counting_rect.0.into(),
          self.display.counting_rect.1.into(),
        );

        self.state = State::Counting;
        self.count_start_time = now();
        true
      }
      EVENT_BLOCKING_END => {
        self.state = State::BlockEnd;
        self.mouse_pos = ctx
          .pointer_latest_pos()
          .map(|p| (p.x as u32, p.y as u32))
          .unwrap_or_default();
        false
      }
      _ => false,
    }
  }

  /// 处理紧急退出逻辑
  fn handle_escape(&mut self, ctx: &eframe::egui::Context, state: State) {
    if matches!(state, State::Blocking) {
      ctx.input(|r| {
        if r.key_pressed(Key::Escape) {
          self.escape_pressed_count += 1;
          if self.escape_pressed_count >= self.escape_count {
            self
              .event_signal
              .store(EVENT_ENTER_COUNTING, std::sync::atomic::Ordering::Relaxed);
          }
        }
      });
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

    let state = self.state;
    let mut blocking_left_secs: u32 = 0;
    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
      ui.style_mut().interaction.selectable_labels = false;

      match state {
        State::Counting => {
          let res = ui.interact(ui.max_rect(), Id::new(1), Sense::drag());
          if res.dragged() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
          }
          let passed_secs =
            (self.count_paused_time.unwrap_or_else(|| now()) - self.count_start_time) as u32;

          let minutes = passed_secs / 60;
          let seconds = passed_secs % 60;

          if passed_secs >= self.work_secs {
            self
              .event_signal
              .store(EVENT_ENTER_BLOCKING, std::sync::atomic::Ordering::Relaxed);
          }

          ui.centered_and_justified(|ui| {
            ui.label(
              RichText::new(format!("{:02}:{:02}", minutes, seconds))
                .monospace()
                .size(COUNTER_WINDOW_FONT_SIZE / self.display.scale)
                .color(Color32::WHITE),
            );
          });
        }
        State::Blocking => {
          ui.vertical_centered(|ui| {
            let passed_secs = (now() - self.count_start_time) as u32;
            let left_secs = self.rest_secs - passed_secs;
            let minutes = left_secs / 60;
            let seconds = left_secs % 60;
            if left_secs <= 0 {
              self
                .event_signal
                .store(EVENT_BLOCKING_END, std::sync::atomic::Ordering::Relaxed);
            } else {
              blocking_left_secs = left_secs;
            }
            let screen_height = self.display.screen_size.1;
            let fsize1 = COUNTER_WINDOW_FONT_SIZE / self.display.scale;
            let fsize2 = fsize1 * 0.86;
            let show_escape = self.escape_pressed_count > (self.escape_count >> 1);
            ui.add_space(
              (screen_height - fsize1 - if show_escape { fsize2 + 12.0 } else { 0.0 }) / 2.0,
            );
            ui.label(
              RichText::new(fmt_count(minutes, seconds))
                .monospace()
                .size(fsize1)
                .color(Color32::WHITE),
            );
            if show_escape {
              ui.add_space(12.0);
              ui.label(
                RichText::new("即将紧急退出")
                  .size(fsize2)
                  .color(Color32::RED),
              );
            }
          });
        }
        State::BlockEnd => {
          let pos = ctx
            .pointer_latest_pos()
            .map(|p| (p.x as u32, p.y as u32))
            .unwrap_or_default();

          if self.mouse_pos.0.abs_diff(pos.0) > 10
            || self.mouse_pos.1.abs_diff(pos.1) > 10
            || ui
              .interact(ui.max_rect(), Id::new(2), Sense::click())
              .clicked()
          {
            self
              .event_signal
              .store(EVENT_ENTER_COUNTING, std::sync::atomic::Ordering::Relaxed);
          }
          ui.centered_and_justified(|ui| {
            ui.label(
              RichText::new("休息结束，移动鼠标解锁~")
                .monospace()
                .size(COUNTER_WINDOW_FONT_SIZE / self.display.scale)
                .color(Color32::WHITE),
            );
          });
        }
      };

      // 渲染第二个屏幕
      if !matches!(state, State::Counting) {
        if let Some(sd) = self.second_display.as_ref() {
          ctx.show_viewport_immediate(
            ViewportId::from_hash_of("second-blocker"),
            ViewportBuilder::default()
              .with_position(sd.screen_rect.0)
              .with_inner_size(sd.screen_rect.1)
              .with_transparent(true)
              .with_decorations(false)
              .with_has_shadow(false)
              .with_icon(IconData::default())
              .with_taskbar(false)
              .with_always_on_top(),
            |ctx, _class| {
              let panel_frame =
                Frame::default().fill(Color32::from_rgba_unmultiplied_const(0, 0, 0, 10));
              let event_signal = self.event_signal.clone();
              CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
                ui.style_mut().interaction.selectable_labels = false;
                if blocking_left_secs == 0
                  && ui
                    .interact(ui.max_rect(), Id::new(2), Sense::click())
                    .clicked()
                {
                  event_signal.store(EVENT_ENTER_COUNTING, std::sync::atomic::Ordering::Relaxed);
                }
                ui.centered_and_justified(|ui| {
                  ui.label(
                    RichText::new(if blocking_left_secs > 0 {
                      let minutes = blocking_left_secs / 60;
                      let seconds = blocking_left_secs % 60;
                      fmt_count(minutes, seconds)
                    } else {
                      "休息结束，点击鼠标解锁~".to_string()
                    })
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
    });

    // 处理紧急退出逻辑
    self.handle_escape(ctx, state);

    if matches!(state, State::Counting | State::Blocking) {
      ctx.request_repaint_after_secs(0.5);
    } else {
      ctx.request_repaint();
    }
  }
}
pub fn start_counter_app(settings: &Settings, event_signal: Arc<AtomicU8>) {
  let counter_app = Box::new(CounterApp::new(settings, event_signal));

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
        .with_has_shadow(false)
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
