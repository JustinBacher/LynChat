<script lang="ts">
  import { onMount } from "svelte"
  import Theme from "../components/common/Theme.svelte"
  import LoadingAnimation from "../components/UI/LoadingAnimation.svelte"
  import ChatContainer from "../components/Chat/ChatContainer.svelte"
  import SettingsMenu from "../components/UI/SettingsMenu.svelte"
  import Hamburger from "../components/UI/Hamburger.svelte"

  let showLoader: boolean = true
  let settingsOpen: boolean = false

  // Sample messages for the demo
  const sampleMessages = [
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
      thoughts: "I should be helpful and direct with calculation requests.",
    },
  ]

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

  <div class="bg-background flex h-screen flex-col">
    <!-- Header -->
    <header
      class="flex items-center justify-between border-b bg-white p-3 shadow-sm"
    >
      <div class="flex items-center">
        <div
          class="bg-primary mr-2 flex h-8 w-8 items-center justify-center rounded-full text-white"
        >
          L
        </div>
        <h1 class="m-0 text-lg font-medium">Lyn AI Assistant</h1>
      </div>

      <Hamburger isOpen={settingsOpen} on:toggle={toggleSettings} />
    </header>

    <!-- Main Content -->
    <main class="bg-chat-pattern h-full flex-1 overflow-hidden">
      <div class="container mx-auto h-full max-w-4xl p-4">
        <ChatContainer messages={sampleMessages} />
      </div>
    </main>

    <!-- Settings Menu (conditionally rendered) -->
    <SettingsMenu bind:isOpen={settingsOpen} />
  </div>
</Theme>
