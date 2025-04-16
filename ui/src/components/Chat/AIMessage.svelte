<script lang="ts">
  import { fade } from "svelte/transition"
  import { quintOut } from "svelte/easing"
  import ThoughtProcess from "./ThoughtProcess.svelte"

  interface Tool {
    name: string
    arguments: Record<string, any>
    result?: Record<string, any>
  }

  interface AIMessageProps {
    text: string
    thoughts?: string | null
    tools?: Tool[]
    timestamp: Date
    thinkingDuration?: number
    isThinking?: boolean
  }

  export let props: AIMessageProps
</script>

<div
  class="mb-4 flex flex-col items-start"
  in:fade={{ duration: 300, easing: quintOut }}
>
  <div class="flex items-start">
    <div
      class="bg-ai-gradient mr-3 flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-xl text-white shadow-lg"
    >
      L
    </div>

    <div class="flex flex-col gap-2">
      <div class="bg-ai-bubble rounded-2xl p-4 shadow-sm">
        <p class="m-0 leading-relaxed">{props.text}</p>
      </div>

      <ThoughtProcess
        props={{
          thoughts: props.thoughts,
          isThinking: props.isThinking,
          thinkingDuration: props.thinkingDuration,
        }}
      />

      {#if props.tools && props.tools.length > 0}
        <!-- Your existing tools code -->
      {/if}
    </div>
  </div>
</div>
