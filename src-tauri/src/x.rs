use serde::de::DeserializeOwned;
use tauri::{
  AppHandle, Runtime,
  plugin::{PluginApi, PluginHandle},
};

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Ios<R>> {
  Ok(Ios(handle))
}

/// Access to the ios APIs.
pub struct Ios<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Ios<R> {
  pub fn start_proxy(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("startProxy", payload)
      .map_err(Into::into)
  }
  pub fn stop_proxy(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("stopProxy", payload)
      .map_err(Into::into)
  }
}
