use tauri::{AppHandle, Manager, Runtime, WebviewWindowBuilder};

use crate::constant::APP_TITLE;

pub fn open_main_window<R: Runtime>(app: &AppHandle<R>) {
  if let Some(x) = app.get_webview_window("main") {
    x.show().unwrap();
    let _ = x.set_focus();
  } else {
    // let cfg = &app.config().app.windows[0];
    let _ = WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("/index.html".into()))
      .title(APP_TITLE)
      .inner_size(400.0, 280.0)
      .center()
      .build()
      .unwrap();
  }
}
