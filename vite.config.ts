import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"
import UnoCSS from "unocss/vite"
import wasmPack from "vite-plugin-wasm-pack"

// https://vitejs.dev/config/
export default defineConfig({
  base: "./",
  plugins: [svelte(), wasmPack("./rust-wasm"), UnoCSS()],
})
