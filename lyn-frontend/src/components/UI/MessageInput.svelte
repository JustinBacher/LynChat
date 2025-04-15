<script lang="ts">
  // Define props interface including event handlers
  interface $$Props {
    onmessage?: (event: CustomEvent<string>) => void
    onsecurityAlert?: (event: CustomEvent<string>) => void
    [key: string]: any // For rest props
  }

  // Export event props
  export let onmessage: ((event: CustomEvent<string>) => void) | undefined =
    undefined
  export let onsecurityAlert:
    | ((event: CustomEvent<string>) => void)
    | undefined = undefined

  let message: string = ""
  let inputElement: HTMLTextAreaElement | undefined

  // Simple patterns to detect potential PII
  const PII_PATTERNS: Record<string, RegExp> = {
    email: /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/,
    phoneNumber: /\b(\+\d{1,3}[ -]?)?\(?\d{3}\)?[ -]?\d{3}[ -]?\d{4}\b/,
    creditCard: /\b(?:\d{4}[ -]?){3}\d{4}\b/,
    ssn: /\b\d{3}[ -]?\d{2}[ -]?\d{4}\b/,
  }

  // Check for PII in the message
  function checkForPII(text: string): void {
    for (const [type, pattern] of Object.entries(PII_PATTERNS)) {
      if (pattern.test(text)) {
        onsecurityAlert?.(
          new CustomEvent("securityAlert", { detail: `potential ${type}` })
        )
        break
      }
    }
  }

  // Send the message
  function sendMessage(): void {
    if (message.trim()) {
      onmessage?.(new CustomEvent("message", { detail: message.trim() }))
      message = ""
      inputElement?.focus()
    }
  }

  // Handle keyboard events
  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault()
      sendMessage()
    }
  }

  // Check for PII as the user types
  function handleInput(): void {
    checkForPII(message)
  }
</script>

<div class="flex">
  <div class="relative flex-1">
    <textarea
      bind:this={inputElement}
      bind:value={message}
      on:input={handleInput}
      on:keydown={handleKeydown}
      class="input min-h-[50px] max-h-[200px] resize-none pr-12"
      placeholder="Type your message..."
      rows="1"
    ></textarea>
    <button
      on:click={sendMessage}
      class="absolute right-2 bottom-2 w-8 h-8 bg-primary text-white rounded-full flex items-center justify-center hover:bg-primary-dark focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
      disabled={!message.trim()}
      aria-label="Send message"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="22" y1="2" x2="11" y2="13"></line>
        <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
      </svg>
    </button>
  </div>
</div>
