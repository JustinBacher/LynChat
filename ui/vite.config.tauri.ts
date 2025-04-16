import { sveltekit } from "@sveltejs/kit/vite"
import { defineConfig } from "vite"

export default defineConfig({
  plugins: [sveltekit()],
  define: {
    __BUILD_TARGET__: JSON.stringify("tauri"),
  },
  build: {
    outDir: "dist/tauri",
    target: "esnext",
    emptyOutDir: true,
  },
})
