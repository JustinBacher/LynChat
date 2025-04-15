<script lang="ts">
  import { fade } from "svelte/transition"

  interface $$Props {
    name: string
    arguments?: Record<string, unknown>
    result?: unknown
  }

  export let name: string
  export let args: Record<string, unknown> = {}
  export let result: unknown = null

  let showDetails: boolean = false

  function formatJSON(obj: unknown): string {
    return JSON.stringify(obj, null, 2)
  }

  function toggleDetails(): void {
    showDetails = !showDetails
  }

  const toolIcons: Record<string, string> = {
    calculator: "🧮",
    datetime: "🕒",
    search: "🔍",
    weather: "☁️",
    file: "📁",
    default: "🔧",
  }

  // Get the appropriate icon
  $: icon = toolIcons[name] || toolIcons.default
</script>

<div class="relative tool-result">
  <div class="flex items-center justify-between">
    <div class="flex items-center">
      <span class="mr-2 text-lg">{icon}</span>
      <span class="font-medium">Used {name}</span>
    </div>
    <button
      class="text-xs text-primary hover:text-primary-dark focus:outline-none"
      on:click={toggleDetails}
    >
      {showDetails ? "Hide details" : "Show details"}
    </button>
  </div>

  {#if showDetails}
    <div in:fade={{ duration: 150 }} class="mt-2">
      <div class="bg-gray-100 p-2 rounded text-xs mb-2">
        <p class="m-0 mb-1 font-semibold">Arguments:</p>
        <pre class="m-0">{formatJSON(args)}</pre>
      </div>

      {#if result !== null}
        <div class="bg-gray-100 p-2 rounded text-xs">
          <p class="m-0 mb-1 font-semibold">Result:</p>
          <pre class="m-0">{typeof result === "object"
              ? formatJSON(result)
              : result}</pre>
        </div>
      {/if}
    </div>
  {/if}
</div>
