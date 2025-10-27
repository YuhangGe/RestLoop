use serde::{Deserialize, Serialize};
use tauri::{App, AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

use crate::constant::{STORE_DATA_PATH, STORE_SETTINGS_KEY};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  #[serde(default = "default_work_secs")]
  pub work_secs: u32,
  #[serde(default = "default_rest_secs")]
  pub rest_secs: u32,
  #[serde(default = "default_escape_count")]
  pub escape_count: u32,
}

pub const DEFAULT_WORK_SECS: u32 = 60 * 30;
pub const DEFAULT_REST_SECS: u32 = 60 * 2;
pub const DEFAULT_ESCAPE_COUNT: u32 = 10;

#[inline]
fn default_work_secs() -> u32 {
  DEFAULT_WORK_SECS
}
#[inline]
fn default_rest_secs() -> u32 {
  DEFAULT_REST_SECS
}
#[inline]
fn default_escape_count() -> u32 {
  DEFAULT_ESCAPE_COUNT
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      work_secs: DEFAULT_WORK_SECS,
      rest_secs: DEFAULT_REST_SECS,
      escape_count: DEFAULT_ESCAPE_COUNT,
    }
  }
}

pub fn setup_settings(app: &mut App) -> Settings {
  let store = app.store(STORE_DATA_PATH).unwrap();

  if let Some(settings) = store.get(STORE_SETTINGS_KEY) {
    serde_json::from_value::<Settings>(settings).unwrap()
  } else {
    let default_settings = Settings::default();
    store.set(
      STORE_SETTINGS_KEY,
      serde_json::to_value(default_settings.clone()).unwrap(),
    );
    default_settings
  }
}

#[tauri::command]
pub async fn tauri_refresh_settings<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
  let store = app.store(STORE_DATA_PATH).unwrap();
  let Some(settings) = store.get(STORE_SETTINGS_KEY) else {
    return Ok(false);
  };

  let Ok(settings) = serde_json::from_value::<Settings>(settings) else {
    return Ok(false);
  };

  Ok(true)
}
