import './App.scss';

import { invoke } from '@tauri-apps/api/core';
import preactLogo from './assets/preact.svg';
import { vm } from 'jinge';

function App() {
  const state = vm({
    name: '',
    greetMsg: '',
  });
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    state.greetMsg = await invoke('greet', { name: state.name });
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + Preact</h1>

      <div className="row">
        <a href="https://vite.dev" rel="noreferrer" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" rel="noreferrer" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://preactjs.com" rel="noreferrer" target="_blank">
          <img src={preactLogo} className="logo preact" alt="Preact logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and Preact logos to learn more.</p>

      <form
        className="row"
        on:submit={(e) => {
          e.preventDefault();
          void greet();
        }}
      >
        <input
          id="greet-input"
          on:input={(e) => (state.name = e.target.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{state.greetMsg}</p>
    </main>
  );
}

export default App;
