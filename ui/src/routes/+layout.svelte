<script lang="ts">
  import isTauri from "$lib/tauri-helpers"
  import "../app.css"

  // INFO: This allows for having the same code for both Tauri and Web
  export const ssr = !isTauri

  let { children } = $props()
</script>

{@render children()}
