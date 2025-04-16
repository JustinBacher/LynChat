<script lang="ts">
  import { fade, slide } from "svelte/transition"

  interface ThoughtProcessProps {
    thoughts: string | null | undefined
    isThinking?: boolean
    thinkingDuration?: number
  }

  export let props: ThoughtProcessProps

  let thoughtState: "collapsed" | "expanded" | "done" = "collapsed"
  let thoughtsContainer: HTMLDivElement

  const thoughtSummary =
    props.thoughts?.split(". ").slice(0, 2).join(". ") + "..."

  function toggleThoughts() {
    thoughtState = thoughtState === "expanded" ? "collapsed" : "expanded"
    return thoughtState
  }

  $: formattedDuration = props.thinkingDuration
    ? `${(props.thinkingDuration / 1000).toFixed(1)}s`
    : null
</script>

{#if props.thoughts}
  {#if props.isThinking}
    <!-- Actively thinking - collapsed view -->
    <div
      class="ml-1 flex items-center gap-2 text-xs text-gray-500"
      in:fade={{ duration: 200 }}
    >
      <div class="flex gap-1">
        <span class="animate-pulse">•</span>
        <span class="animate-pulse" style="animation-delay: 200ms">•</span>
        <span class="animate-pulse" style="animation-delay: 400ms">•</span>
      </div>
      <span class="italic">{thoughtSummary}</span>
    </div>
  {:else if thoughtState === "collapsed"}
    <!-- Done thinking - collapsed view -->
    <button
      class="ml-1 flex items-center gap-1.5 text-xs text-gray-400 hover:text-gray-600"
      on:click={toggleThoughts}
    >
      <svg
        class="h-3 w-3"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 4.5v15m6-15v15m-6-9h6"
        />
      </svg>
      <span class="font-mono">{formattedDuration}</span>
    </button>
  {:else}
    <!-- Expanded thinking view -->
    <div
      class="ml-1 max-h-48 overflow-y-auto rounded-lg bg-gray-50 p-3 text-sm dark:bg-gray-800/50"
      bind:this={thoughtsContainer}
      in:slide={{ duration: 200 }}
    >
      <div class="mb-2 flex items-center justify-between text-xs text-gray-400">
        <span class="font-mono">{formattedDuration}</span>
        <button
          class="hover:text-gray-600"
          on:click={toggleThoughts}
          aria-label="Collapse"
        >
          <svg
            class="h-3 w-3"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
      <div class="prose prose-sm max-w-none text-gray-600 dark:text-gray-300">
        {props.thoughts}
      </div>
    </div>
  {/if}
{/if}

<style>
  div::-webkit-scrollbar {
    width: 6px;
  }

  div::-webkit-scrollbar-track {
    background: transparent;
  }

  div::-webkit-scrollbar-thumb {
    background-color: rgba(156, 163, 175, 0.5);
    border-radius: 3px;
  }
</style>
