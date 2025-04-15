<script lang="ts">
  import { onMount } from "svelte"

  interface $$Props {
    duration?: number
    autoStart?: boolean
    showOverlay?: boolean
  }

  export let duration: number = 1000 // Default animation duration
  export let autoStart: boolean = true // Auto start animation
  export let showOverlay: boolean = true // Show dark overlay

  let visible: boolean = true

  onMount(() => {
    if (autoStart) {
      setTimeout(() => {
        visible = false
      }, duration)
    }
  })

  // Function to manually hide the animation
  export function hide(): void {
    visible = false
  }
</script>

{#if visible}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    {#if showOverlay}
      <div class="absolute inset-0 bg-black bg-opacity-50"></div>
    {/if}

    <div class="relative z-10 flex flex-col items-center">
      <div class="w-24 h-24 mb-4">
        <!-- Logo or branding element -->
        <div
          class="w-full h-full rounded-full bg-primary flex items-center justify-center text-white text-3xl font-bold animate-pulse"
        >
          L
        </div>
      </div>

      <!-- Text elements with staggered blur-focus animations -->
      <div class="text-white text-2xl font-medium blur-focus">
        Lyn AI Assistant
      </div>

      <div
        class="text-white text-sm mt-2 blur-focus"
        style="animation-delay: 100ms"
      >
        Your privacy-first AI companion
      </div>

      <div
        class="mt-8 w-32 h-1 bg-white bg-opacity-20 rounded overflow-hidden blur-focus"
        style="animation-delay: 200ms"
      >
        <div
          class="h-full w-1/2 bg-accent-orange rounded"
          style="animation: loading 1s infinite linear"
        ></div>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes loading {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(200%);
    }
  }
</style>
