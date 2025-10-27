use tauri::{
  App, Manager,
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
};

use crate::{
  constant::APP_TITLE,
  window_counter::{
    CounterEventSignal, EVENT_PAUSE_COUNTING, EVENT_RESET_COUNTING, EVENT_RESUME_COUNTING,
  },
  window_main::open_main_window,
};

const TRAY_MENU_QUIT: &'static str = "quit";
const TRAY_MENU_RESET: &'static str = "reset";
const TRAY_MENU_PAUSE: &'static str = "puase";
const TRAY_MENU_RESUME: &'static str = "resume";
const TRAY_MENU_SETTING: &'static str = "setting";

pub fn setup_tray(app: &mut App) {
  let quit_i = MenuItem::with_id(app, TRAY_MENU_QUIT, "退出", true, None::<&str>).unwrap();
  let reset_i = MenuItem::with_id(app, TRAY_MENU_RESET, "重置", true, None::<&str>).unwrap();
  let pause_i = MenuItem::with_id(app, TRAY_MENU_PAUSE, "暂停", true, None::<&str>).unwrap();
  let resume_i = MenuItem::with_id(app, TRAY_MENU_RESUME, "继续", true, None::<&str>).unwrap();
  let setting_i = MenuItem::with_id(app, TRAY_MENU_SETTING, "设置", true, None::<&str>).unwrap();

  let menu = Menu::with_items(app, &[&pause_i, &resume_i, &reset_i, &setting_i, &quit_i]).unwrap();
  let _ = TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu)
    .tooltip(APP_TITLE)
    .show_menu_on_left_click(true)
    // .on_tray_icon_event(|ic, event| {
    //   use tauri::tray::TrayIconEvent;
    //   if let TrayIconEvent::Click { button, .. } = &event {
    //     use tauri::tray::MouseButton;
    //     if matches!(button, MouseButton::Left) {
    //       open_main_window(ic.app_handle());
    //     }
    //   }
    // })
    .on_menu_event(move |app, event| match event.id.as_ref() {
      TRAY_MENU_QUIT => {
        app.exit(0);
      }
      TRAY_MENU_PAUSE => {
        app
          .state::<CounterEventSignal>()
          .store(EVENT_PAUSE_COUNTING, std::sync::atomic::Ordering::Relaxed);
      }
      TRAY_MENU_RESUME => {
        app
          .state::<CounterEventSignal>()
          .store(EVENT_RESUME_COUNTING, std::sync::atomic::Ordering::Relaxed);
      }
      TRAY_MENU_SETTING => {
        open_main_window(app);
      }
      TRAY_MENU_RESET => {
        app
          .state::<CounterEventSignal>()
          .store(EVENT_RESET_COUNTING, std::sync::atomic::Ordering::Relaxed);
      }
      _ => {
        println!("menu item {:?} not handled", event.id);
      }
    })
    .build(app)
    .unwrap();
}
