import {
  Button,
  Controller,
  InputAddon,
  InputNumber,
  InputWrapper,
  message,
  useForm,
} from 'jinge-antd';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';

import { FormItem } from './components/FormItem';
import { Switch } from './components/Switch';
import { globalSettings } from './store/settings';
import { invoke } from '@tauri-apps/api/core';
import z from 'zod';

function App() {
  const { formErrors, validate, control } = useForm(
    z.object({
      workMinutes: z.number().int().min(1).max(60),
      autoStartApp: z.boolean(),
      restMinutes: z.number().int().min(1).max(30),
    }),
    {
      defaultValues: {
        autoStartApp: globalSettings.autoStartApp,
        workMinutes: globalSettings.workSecs / 60,
        restMinutes: globalSettings.restSecs / 60,
      },
    },
  );
  console.log(globalSettings);
  async function save() {
    const [err, data] = await validate();
    if (err) return;

    const oldAutoStart = globalSettings.autoStartApp;
    if (oldAutoStart !== data.autoStartApp) {
      globalSettings.autoStartApp = data.autoStartApp;
      if (data.autoStartApp) {
        if (!(await isEnabled())) {
          await enable();
        }
        message.success('已配置开机启动！');
      } else {
        if (await isEnabled()) {
          await disable();
        }
        message.success('已取消开机启动！');
      }
    }

    const workSecs = data.workMinutes * 60;
    const restSecs = data.restMinutes * 60;
    if (
      workSecs !== globalSettings.workSecs ||
      restSecs !== globalSettings.restSecs
    ) {
      globalSettings.workSecs = workSecs;
      globalSettings.restSecs = restSecs;
      await invoke('tauri_refresh_settings');
      message.success('保存成功！');
    }
  }

  return (
    <main className="p-4">
      <h1 className="font-medium text-2xl">参数配置</h1>
      <div className="mt-6 flex max-w-md flex-col gap-6 text-sm max-sm:max-w-full">
        <FormItem label="工作时长：" error={formErrors.workMinutes}>
          <Controller control={control} name="workMinutes">
            {(field) => (
              <InputWrapper>
                <InputNumber
                  noRoundedR
                  step={1}
                  min={10}
                  max={60}
                  value={field.value}
                  on:change={(v) => {
                    field['on:change'](v);
                  }}
                />
                <InputAddon className="shrink-0 whitespace-nowrap px-2">
                  分钟
                </InputAddon>
              </InputWrapper>
            )}
          </Controller>
        </FormItem>
        <FormItem label="休息时长：" error={formErrors.restMinutes}>
          <Controller control={control} name="restMinutes">
            {(field) => (
              <InputWrapper>
                <InputNumber
                  noRoundedR
                  step={1}
                  min={1}
                  max={30}
                  value={field.value}
                  on:change={(v) => {
                    field['on:change'](v);
                  }}
                />
                <InputAddon className="shrink-0 whitespace-nowrap px-2">
                  分钟
                </InputAddon>
              </InputWrapper>
            )}
          </Controller>
        </FormItem>
        <FormItem label="开机启动：" error={formErrors.autoStartApp}>
          <Controller control={control} name="autoStartApp">
            {(field) => (
              <div className="flex items-center">
                <Switch
                  value={field.value}
                  on:change={(checked) => {
                    field['on:change'](checked);
                  }}
                />
              </div>
            )}
          </Controller>
        </FormItem>
        <div className="mt-2 flex items-center gap-8">
          <Button
            type="primary"
            on:click={() => {
              void save();
            }}
          >
            保存
          </Button>
        </div>
      </div>
    </main>
  );
}

export default App;
