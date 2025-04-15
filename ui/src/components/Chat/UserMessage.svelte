<!-- ui/src/components/Chat/UserMessage.svelte -->
<script lang="ts">
  export let text
  export let timestamp = new Date()

  function formatTime(date: Date): string {
    return date.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    })
  }
</script>

<div class="mb-4 flex flex-col items-end">
  <div
    class="bg-accent-orange shadow-card max-w-[80%] rounded-lg p-4 text-white"
  >
    <p class="m-0">{text}</p>
  </div>
  <span class="mt-1 text-xs text-gray-500">{formatTime(timestamp)}</span>
</div>
