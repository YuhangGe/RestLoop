import { DefaultSettings, type Settings } from '@/service/settings';
import { type Store, load } from '@tauri-apps/plugin-store';
import { vm, vmRaw, vmWatch } from 'jinge';

import { currentInWebMock } from '@/service/util';

let tauriSettingStore: Store | undefined = undefined;

export const globalSettings = vm<Settings>({
  ...DefaultSettings,
});

export async function loadGlobalSettings() {
  if (currentInWebMock) return;

  tauriSettingStore = await load('app_data.json', {
    defaults: {},
    autoSave: true,
  });

  const settings = await tauriSettingStore.get('settings');
  Object.assign(globalSettings, settings);

  let tm = 0;
  vmWatch(globalSettings, () => {
    if (tm) clearTimeout(tm);
    tm = window.setTimeout(() => {
      tm = 0;
      void tauriSettingStore?.set('settings', vmRaw(globalSettings));
    });
  });
}
