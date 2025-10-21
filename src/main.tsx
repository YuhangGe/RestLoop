import './style.css';

import App from './App';
import { bootstrap } from 'jinge';
import { loadGlobalSettings } from './store/settings';

const root = document.querySelector('#root')!;
if (!root) throw new Error('#root not found');

window.onunhandledrejection = (evt) => {
  console.error(evt);
};
window.onerror = (evt) => {
  console.error(evt);
};

void loadGlobalSettings().then(() => {
  bootstrap(App, root as HTMLElement);
});
