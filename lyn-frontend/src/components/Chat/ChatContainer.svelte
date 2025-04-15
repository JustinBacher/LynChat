<script lang="ts">
  import type { Writable } from "svelte/store"
  import { writable } from "svelte/store"
  import UserMessage from "./UserMessage.svelte"
  import AIMessage from "./AIMessage.svelte"
  import MessageInput from "../UI/MessageInput.svelte"
  import SecurityMascot from "../UI/SecurityMascot.svelte"
  import type { Message } from "../../stores/chat"

  interface $$Props {
    messages?: Message[]
  }

  // Sample data for mock messages
  let { messages } = $props()

  // Store for detected PII
  const detectedPII: Writable<{ type: string; value: unknown } | null> =
    writable(null)

  // Reference to the chat container for auto-scrolling
  let chatContainer: HTMLElement
  let autoScroll: boolean = true

  // Add a new user message
  function addUserMessage(text: string): void {
    messages = [
      ...messages,
      { id: "3", type: "user", text, timestamp: new Date() },
    ]

    // Simulate AI response with a delay
    setTimeout(() => {
      const aiResponse: Message = {
        id: "4",
        type: "ai",
        text: "This is a sample AI response. In a real implementation, this would come from the backend.",
        thoughts: "I am thinking about how to respond to the user query.",
        tools: [
          {
            name: "calculator",
            arguments: { expression: "2+2" },
            result: "4",
          },
        ],
        timestamp: new Date(),
      }

      messages = [...messages, aiResponse]
    }, 1000)
  }

  // Handle security alerts
  function handleSecurityAlert(event: CustomEvent<string>): void {
    detectedPII.set({ type: event.detail, value: null })
  }

  // After any update, scroll to bottom if autoScroll is true
  $effect(() => {
    if (autoScroll && chatContainer) {
      chatContainer.scrollTop = chatContainer.scrollHeight
    }
  })
</script>

<div class="flex flex-col h-full">
  <!-- Chat Messages Area -->
  <div
    class="flex-1 overflow-y-auto p-4"
    bind:this={chatContainer}
    "on:scroll"={handleScroll}
  >
    {#each messages as message, i (i)}
      <div class="blur-focus" style="animation-delay: {i * 100}ms">
        {#if message.type === "user"}
          <UserMessage text={message.text} timestamp={message.timestamp} />
        {:else if message.type === "ai"}
          <AIMessage
            text={message.text}
            thoughts={message.thoughts}
            tools={message.tools}
            timestamp={message.timestamp}
          />
        {/if}
      </div>
    {/each}

    {#if messages.length === 0}
      <div
        class="h-full flex items-center justify-center text-gray-400 blur-focus"
      >
        <p class="text-center">Start a conversation with Lyn</p>
      </div>
    {/if}
  </div>

  <!-- Input Area -->
  <div class="border-t p-4 bg-white">
    <MessageInput
      on:message={(e) => addUserMessage(e.detail)}
      on:securityAlert={handleSecurityAlert}
    />
  </div>

  <!-- Security Mascot -->
  <SecurityMascot bind:detectedPII={$detectedPII} />
</div>
