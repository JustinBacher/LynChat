import adapterAuto from "@sveltejs/adapter-auto"
import adapterStatic from "@sveltejs/adapter-static"
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte"

/** @type {import('@sveltejs/kit').Config} */
const config = {
  prerender: {
    default: false,
  },
  preprocess: vitePreprocess(),
  kit: {
    adapter:
      process.env.ADAPTER === "static"
        ? adapterStatic({
            fallback: "index.html",
            pages: "dist/tauri",
            assets: "dist/tauri",
          })
        : adapterAuto(),
    env: {
      publicPrefix: "PUBLIC_",
    },
  },
}

export default config
