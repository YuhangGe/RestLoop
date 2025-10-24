use serde::{Deserialize, Serialize};
use tauri::{App, AppHandle, Runtime, State};
use tauri_plugin_store::StoreExt;

use crate::constant::{STORE_DATA_PATH, STORE_SETTINGS_KEY};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  pub work_secs: u32,
  pub rest_secs: u32,
}

pub const DEFAULT_WORK_SECS: u32 = 60 * 30;
pub const DEFAULT_REST_SECS: u32 = 60 * 2;

impl Default for Settings {
  fn default() -> Self {
    Settings {
      work_secs: DEFAULT_WORK_SECS,
      rest_secs: DEFAULT_REST_SECS,
    }
  }
}

impl Settings {
  pub fn save(&self) {}
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
pub async fn tauri_load_settings<R: Runtime>(
  _app: AppHandle<R>,
  state: State<'_, Settings>,
) -> Result<Settings, String> {
  let x = state.inner().clone();
  Ok(x)
}
