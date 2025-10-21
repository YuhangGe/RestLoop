import { platform } from '@tauri-apps/plugin-os';

export const currentPlatform = platform();
/**
 * 当前是否在纯浏览器的 web 模拟环境。比如没有启动 rust 后端时直接通过浏览器访问。
 */
export const currentInWebMock = (currentPlatform as string) === 'webmock';

export function uid() {
  return (
    Date.now().toString(32) + Math.floor(Math.random() * 0xffffff).toString(32)
  );
}

export const IS_RELOAD = sessionStorage.getItem('devreload') === '1';
if (!IS_RELOAD) {
  sessionStorage.setItem('devreload', '1');
}
export const IS_REOPEN = location.search.includes('mode=reopen');

export const IS_MOBILE =
  currentPlatform === 'android' || currentPlatform === 'ios';
export const IS_IOS = currentPlatform === 'ios';
export const IS_ANDROID = currentPlatform === 'android';
