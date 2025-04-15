<script lang="ts">
  import { onMount } from "svelte"
  import Theme from "../components/common/Theme.svelte"
  import LoadingAnimation from "../components/UI/LoadingAnimation.svelte"
  import ChatContainer from "../components/Chat/ChatContainer.svelte"
  import SettingsMenu from "../components/UI/SettingsMenu.svelte"
  import Hamburger from "../components/UI/Hamburger.svelte"
  import type { Message } from "../stores/chat"

  let showLoader: boolean = true
  let settingsOpen: boolean = false

  // Sample messages for demo
  const sampleMessages: Message[] = [
    {
      id: "1",
      type: "user",
      text: "Hello! Can you help me with a calculation?",
      timestamp: new Date(Date.now() - 60000), // 1 minute ago
    },
    {
      id: "2",
      type: "ai",
      text: "Of course! I'd be happy to help with a calculation. What would you like to calculate?",
      timestamp: new Date(Date.now() - 45000), // 45 seconds ago
    },
  ]

  // Remove loader after animation completes
  onMount(() => {
    setTimeout(() => {
      showLoader = false
    }, 1500)
  })

  function toggleSettings(): void {
    settingsOpen = !settingsOpen
  }
</script>

<Theme>
  {#if showLoader}
    <LoadingAnimation duration={1500} />
  {/if}

  <div class="flex flex-col h-screen bg-background">
    <!-- Header -->
    <header
      class="bg-white border-b p-3 flex justify-between items-center shadow-sm"
    >
      <div class="flex items-center">
        <div
          class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center mr-2"
        >
          L
        </div>
        <h1 class="text-lg font-medium m-0">Lyn AI Assistant</h1>
      </div>

      <Hamburger isOpen={settingsOpen} on:toggle={toggleSettings} />
    </header>

    <!-- Main Content -->
    <main class="flex-1 overflow-hidden h-full">
      <ChatContainer messages={sampleMessages} />
    </main>

    <!-- Settings Menu (conditionally rendered) -->
    <SettingsMenu bind:isOpen={settingsOpen} />
  </div>
</Theme>
