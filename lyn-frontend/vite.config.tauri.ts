import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig(({ mode }) => {
  return {
    plugins: [sveltekit()],
    define: {
      __BUILD_TARGET__: JSON.stringify(mode),
    },
    build: {
      outDir: "../dist/tauri",
      // Tauri uses this to inline assets - important!
      target: "esnext",
      minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
      sourcemap: !!process.env.TAURI_DEBUG,
    },
  };
});
