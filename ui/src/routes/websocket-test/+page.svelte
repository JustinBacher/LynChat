<script lang="ts">
    import { websocketStore } from '$lib/stores/websocketStore';
    import { onMount, onDestroy } from 'svelte';

    // State using runes
    let messages = $state<{text: string, sent: boolean}[]>([]);
    let messageInput = $state('');
    let connectionStatus = $state('Disconnected');

    // Handle incoming messages
    function handleWebSocketMessage(event: CustomEvent) {
        const message = event.detail;
        messages = [...messages, { text: message, sent: false }];
    }

    // Send a message
    function sendMessage() {
        if (!messageInput.trim()) return;

        if (websocketStore.isConnected()) {
            websocketStore.send(messageInput);
            messages = [...messages, { text: messageInput, sent: true }];
            messageInput = '';
        } else {
            alert('WebSocket is not connected');
        }
    }

    // Connect to WebSocket
    function connect() {
        websocketStore.connect();
    }

    // Disconnect from WebSocket
    function disconnect() {
        websocketStore.disconnect();
    }

    // Update connection status when the store changes
    const unsubscribe = websocketStore.subscribe(state => {
        connectionStatus = state.connected ? 'Connected' : 'Disconnected';
        if (state.error) {
            connectionStatus += ` (Error: ${state.error})`;
        }
    });

    // Setup event listeners
    onMount(() => {
        // Listen for WebSocket messages
        window.addEventListener('ws-message', handleWebSocketMessage as EventListener);
    });

    // Cleanup
    onDestroy(() => {
        unsubscribe();
        window.removeEventListener('ws-message', handleWebSocketMessage as EventListener);
    });
</script>

<div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4">WebSocket Test</h1>

    <div class="mb-4">
        <p class="mb-2">Status: <span class={connectionStatus === 'Connected' ? 'text-green-500' : 'text-red-500'}>{connectionStatus}</span></p>
        <div class="flex gap-2">
            <button class="btn btn-primary" onclick={connect}>Connect</button>
            <button class="btn btn-secondary" onclick={disconnect}>Disconnect</button>
        </div>
    </div>

    <div class="mb-4">
        <div class="flex gap-2">
            <input
                type="text"
                bind:value={messageInput}
                placeholder="Type a message..."
                class="input input-bordered w-full"
                onkeydown={(e) => e.key === 'Enter' && sendMessage()}
            />
            <button class="btn btn-primary" onclick={sendMessage}>Send</button>
        </div>
    </div>

    <div class="border rounded-lg p-4 h-96 overflow-y-auto bg-base-200">
        <h2 class="text-xl mb-2">Messages</h2>
        {#if messages.length === 0}
            <p class="text-gray-500">No messages yet</p>
        {:else}
            <div class="flex flex-col gap-2">
                {#each messages as message}
                    <div class={`p-2 rounded-lg ${message.sent ? 'bg-primary text-primary-content self-end' : 'bg-secondary text-secondary-content self-start'}`}>
                        {message.text}
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>
