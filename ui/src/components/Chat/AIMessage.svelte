<script lang="ts">
  import { fade } from "svelte/transition"
  import ThoughtProcess from "./ThoughtProcess.svelte"
  import ToolCall from "./ToolCall.svelte"

  interface Tool {
    name: string
    arguments: Record<string, unknown>
    result: unknown
  }

  interface $$Props {
    text: string
    thoughts?: string | null
    tools?: Tool[]
    timestamp?: Date
  }

  export let text: string
  export let thoughts: string | null = null
  export let tools: Tool[] = []
  export let timestamp: Date = new Date()

  let showThoughts: boolean = false

  function formatTime(date: Date): string {
    return date.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    })
  }

  function toggleThoughts(): void {
    showThoughts = !showThoughts
  }
</script>

<div class="flex flex-col items-start mb-4">
  <div class="flex items-start">
    <div
      class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center mr-2 flex-shrink-0"
    >
      L
    </div>
    <div>
      <div class="ai-message">
        <p class="m-0">{text}</p>
      </div>

      {#if thoughts}
        <button
          class="text-xs text-primary hover:text-primary-dark mt-1 ml-2 focus:outline-none"
          on:click={toggleThoughts}
        >
          {showThoughts ? "Hide thinking" : "Show thinking"}
        </button>
      {/if}

      {#if showThoughts && thoughts}
        <div in:fade={{ duration: 150 }}>
          <ThoughtProcess text={thoughts} />
        </div>
      {/if}

      {#each tools as tool, i}
        <div class="mt-2" in:fade={{ duration: 150, delay: 100 * i }}>
          <ToolCall
            name={tool.name}
            arguments={tool.arguments}
            result={tool.result}
          />
        </div>
      {/each}
      <span class="text-xs text-gray-500 mt-1 block"
        >{formatTime(timestamp)}</span
      >
    </div>
  </div>
</div>
