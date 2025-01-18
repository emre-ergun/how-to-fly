import wasm from "vite-plugin-wasm";
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
  server: {
    port: 3000,
  }
})
