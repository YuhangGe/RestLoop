mod chinese_font;
mod constant;
mod settings;
mod test;
mod tray;
mod util;
mod window_counter;
mod window_main;

use tauri::Manager;

use crate::settings::{setup_settings, tauri_load_settings};
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
    .invoke_handler(tauri::generate_handler![tauri_load_settings])
    .setup(|app| {
      setup_tray(app);
      let settings = setup_settings(app);

      start_counter_app(&settings);

      // std::thread::spawn(move || {
      //   loop {
      //     match rx.recv() {
      //       Ok(Message::CounterEnd) => {
      //         counter.lock().unwrap().close();
      //         blocker.lock().unwrap().start(sx.clone());
      //       }
      //       Ok(Message::BlockerEnd) => {
      //         blocker.lock().unwrap().close();
      //         counter.lock().unwrap().start(sx.clone());
      //       }
      //       _ => (),
      //     }
      //   }
      // });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
