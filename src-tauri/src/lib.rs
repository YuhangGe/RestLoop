mod chinese_font;
mod constant;
mod settings;
mod tray;
mod window_counter;
mod window_main;

use std::sync::Arc;
use std::sync::atomic::AtomicU8;

use tauri::Manager;

use crate::settings::{setup_settings, tauri_refresh_settings};
use crate::tray::setup_tray;
use crate::window_counter::start_counter_app;

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
    .invoke_handler(tauri::generate_handler![tauri_refresh_settings])
    .setup(|app| {
      setup_tray(app);
      let settings = setup_settings(app);
      let event_signal = Arc::new(AtomicU8::new(0));
      app.manage(event_signal.clone());

      start_counter_app(&settings, event_signal);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
