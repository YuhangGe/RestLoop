use tauri::{
  App,
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
};

use crate::{constant::APP_TITLE, window_main::open_main_window};

pub fn setup_tray(app: &mut App) {
  let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
  let menu = Menu::with_items(app, &[&quit_i]).unwrap();
  let _ = TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu)
    .tooltip(APP_TITLE)
    .show_menu_on_left_click(false)
    .on_tray_icon_event(|ic, event| {
      use tauri::tray::TrayIconEvent;

      if let TrayIconEvent::Click { button, .. } = &event {
        use tauri::tray::MouseButton;

        if matches!(button, MouseButton::Left) {
          open_main_window(ic.app_handle());
        }
      }
    })
    .on_menu_event(|app, event| match event.id.as_ref() {
      "quit" => {
        app.exit(0);
      }
      _ => {
        println!("menu item {:?} not handled", event.id);
      }
    })
    .build(app)
    .unwrap();
}
