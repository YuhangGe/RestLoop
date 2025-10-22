mod constant;
mod counter;
mod settings;
mod tray;
mod window_counter;
mod window_main;
mod x;

use tauri::Manager;
use tauri::plugin::Builder;
use tokio::sync;
use winit::raw_window_handle::HasDisplayHandle;

use crate::counter::Counter;
use crate::settings::{setup_settings, tauri_load_settings};
use crate::tray::setup_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      None,
    ))
    .plugin()
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
    .invoke_handler(tauri::generate_handler![tauri_load_settings])
    .setup(|app| {
      setup_tray(app);
      let settings = setup_settings(app);
      let counter = Counter::new_state(settings.work_secs);
      app.manage(counter.clone());

      tauri::async_runtime::spawn(async move {
        // counter.lock().await.start().await;
      });
      let x = tauri::window::WindowBuilder::new(app, "xxx")
        .title("xxx")
        .build()
        .unwrap();

      x.show().unwrap();

      let h = x.display_handle().unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
