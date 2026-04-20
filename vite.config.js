import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  root: 'src/renderer',
  plugins: [react()],
  build: {
    outDir: path.resolve(__dirname, 'dist/renderer'),
    emptyOutDir: true,
  },
  server: {
    port: 5173,
  },
  resolve: {
    alias: {
      '@crawler': path.resolve(__dirname, 'src/crawler'),
      '@deobfuscator': path.resolve(__dirname, 'src/deobfuscator'),
      '@ai': path.resolve(__dirname, 'src/ai'),
      '@crypto': path.resolve(__dirname, 'src/crypto'),
    },
  },
});
