import { TailwindThemePlugin } from './vite-plugin';
import { defineConfig } from 'vite';
import imagemin from 'vite-plugin-minipic';
import { jingeVitePlugin } from 'jinge-compiler';
import { networkInterfaces } from 'node:os';
import path from 'node:path';
import tailwindcss from '@tailwindcss/vite';
import tsconfigPaths from 'vite-tsconfig-paths';

let IPv4 = '';
Object.entries(networkInterfaces()).some(([, nets]) => {
  if (!nets) return false;
  return nets.some((net) => {
    // Skip over non-IPv4 and internal (i.e. 127.0.0.1) addresses
    // 'IPv4' is in Node <= 17, from 18 it's a number 4 or 6
    const familyV4Value = typeof net.family === 'string' ? 'IPv4' : 4;
    if (net.family === familyV4Value && !net.internal) {
      IPv4 = net.address;
      return true;
    } else {
      return false;
    }
  });
});

// https://vitejs.dev/config/
export default defineConfig({
  envDir: __dirname,
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: '0.0.0.0',
    hmr: {
      protocol: 'ws',
      host: IPv4,
      port: 1421,
    },
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
  optimizeDeps: {
    exclude: ['jinge-antd'],
  },
  resolve: {
    alias: {
      'jinge-antd': 'jinge-antd/source',
    },
  },
  build: {
    target: 'esnext',
    rollupOptions: {
      input: {
        index: path.resolve(__dirname, '../index.html'),
      },
    },
  },
  plugins: [
    tailwindcss(),
    tsconfigPaths({
      projects: [path.resolve(__dirname, '../tsconfig.json')],
    }),
    TailwindThemePlugin(),
    jingeVitePlugin(),
    imagemin(),
    tsconfigPaths({
      projects: [path.resolve(__dirname, '../tsconfig.json')],
    }),
  ],
});
