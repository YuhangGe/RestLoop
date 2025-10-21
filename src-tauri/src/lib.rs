mod counter;

use std::thread;

use tauri::{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
  AppHandle, Manager, Runtime, WebviewWindowBuilder,
};

use crate::counter::show_counter_window;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

const APP_TITLE: &'static str = "健康休息提醒器";

fn open_main_window<R: Runtime>(app: &AppHandle<R>) {
  if let Some(x) = app.get_webview_window("main") {
    x.show().unwrap();
    let _ = x.set_focus();
  } else {
    // let cfg = &app.config().app.windows[0];
    let _ = WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("/index.html".into()))
      .title(APP_TITLE)
      .inner_size(800.0, 600.0)
      .build()
      .unwrap();
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      None,
    ))
    .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
      // let win = app.get_webview_window("main").expect("no main window");
      // win.show().unwrap();
      // let _ = win.set_focus();
      let Some(win) = app.get_webview_window("main") else {
        return;
      };
      let _ = win.show();
      let _ = win.set_focus();
    }))
    .invoke_handler(tauri::generate_handler![greet])
    .setup(|_app| {
      let quit_i = MenuItem::with_id(_app, "quit", "退出", true, None::<&str>).unwrap();
      let menu = Menu::with_items(_app, &[&quit_i]).unwrap();
      let _ = TrayIconBuilder::new()
        .icon(_app.default_window_icon().unwrap().clone())
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
        .build(_app)
        .unwrap();

      thread::spawn(|| {
        show_counter_window();
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
