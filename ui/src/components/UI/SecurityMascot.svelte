<script lang="ts">
  // TODO: Needs to see if fade is needded here
  // import { fade, fly } from "svelte/transition";
  import { fly } from "svelte/transition"

  interface PIIData {
    id: string
    type: string
    value: unknown
  }

  interface $$Props {
    detectedPII: PIIData | null
  }

  export let detectedPII: PIIData | null = null

  function markAsPrivate(): void {
    // In a real implementation, this would mask the sensitive data
    alert(
      "Data marked as private. It will be redacted before sending to any service."
    )
    detectedPII = null
  }

  function markAsPublic(): void {
    // In a real implementation, this would allow the data to be sent
    alert(
      "Data marked as public. Please be careful with sensitive information."
    )
    detectedPII = null
  }
</script>

{#if detectedPII}
  <div
    class="fixed right-5 bottom-5 z-50 max-w-md"
    in:fly={{ y: 20, duration: 200 }}
    out:fly={{ y: 20, duration: 200 }}
  >
    <div
      class="shadow-elevated border-accent-orange rounded-lg border-l-4 bg-white p-4"
    >
      <div class="flex">
        <!-- Security Mascot Icon -->
        <div class="mr-3 flex-shrink-0">
          <div
            class="bg-primary flex h-12 w-12 animate-pulse items-center justify-center rounded-full text-2xl text-white"
          >
            🛡️
          </div>
        </div>

        <div class="flex-1">
          <h4 class="mb-1 text-base font-medium text-gray-900">
            Potential Privacy Concern
          </h4>
          <p class="mb-3 text-sm text-gray-700">
            I've detected what appears to be {detectedPII.type} in your message.
            How would you like to proceed?
          </p>

          <div class="flex space-x-2">
            <button
              class="btn-primary px-3 py-1 text-xs"
              on:click={markAsPrivate}
            >
              Make Private
            </button>
            <button
              class="btn-secondary px-3 py-1 text-xs"
              on:click={markAsPublic}
            >
              Keep Public
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
