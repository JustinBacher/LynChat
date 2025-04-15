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

<div class="mb-4 flex flex-col items-start">
  <div class="flex items-start">
    <div
      class="bg-primary mr-2 flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full text-white"
    >
      L
    </div>
    <div>
      <div class="bg-ai-bubble shadow-card rounded-lg p-4">
        <p class="m-0">{text}</p>
      </div>

      {#if thoughts}
        <button
          class="text-primary hover:text-primary-dark mt-1 ml-2 text-xs focus:outline-none"
          on:click={toggleThoughts}
        >
          {showThoughts ? "Hide thinking" : "Show thinking"}
        </button>
      {/if}

      {#if showThoughts && thoughts}
        <div in:fade={{ duration: 150 }}>
          <div class="bg-thought-bubble mt-2 rounded-lg p-3 text-sm italic">
            <ThoughtProcess text={thoughts} />
          </div>
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
      <span class="mt-1 block text-xs text-gray-500"
        >{formatTime(timestamp)}</span
      >
    </div>
  </div>
</div>
