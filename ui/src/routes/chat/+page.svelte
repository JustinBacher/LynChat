<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { slide } from 'svelte/transition';
	import ChatBox from '$lib/components/ChatBox.svelte';
	import type { Message } from '$lib/components/ChatBox.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import { websocketStore, type WebSocketState } from '$lib/stores/websocketStore';
	import { initialMessageStore } from '$lib/stores/chatStore';

	const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'https://api.lyn.ai';

	// Get the initial message from the store
	let initialMessage = '';
	const unsubscribeInitialMessage = initialMessageStore.subscribe((value: string) => {
		initialMessage = value;
	});

	let messages: Message[] = [
		{ text: 'Hello! How can I help you today?', sender: 'ai', isError: false }
	];

	// Subscribe to the WebSocket store
	let wsState: WebSocketState = { connected: false, error: null };
	const unsubscribe = websocketStore.subscribe(state => {
		wsState = state;
	});

	// Handle WebSocket messages
	function handleWebSocketMessage(event: CustomEvent) {
		try {
			const response = JSON.parse(event.detail);
			if (response.error) {
				handleError(response.error);
				return;
			}

			messages = [...messages, { text: response.content, sender: 'ai', isError: false }];
		} catch (error) {
			console.error('Error parsing WebSocket message:', error);
		}
	}

	onMount(() => {
		// Connect to WebSocket
		websocketStore.connect();

		// Add event listener for WebSocket messages
		window.addEventListener('ws-message', handleWebSocketMessage as EventListener);

		// Process initial message if it exists
		if (initialMessage.trim()) {
			// Add a slight delay to ensure the transition completes first
			setTimeout(() => {
				handleNewMessage(initialMessage);
				// Clear the store after using it
				initialMessageStore.set('');
			}, 500);
		}
	});

	onDestroy(() => {
		// Clean up subscriptions and event listeners
		unsubscribe();
		unsubscribeInitialMessage();
		window.removeEventListener('ws-message', handleWebSocketMessage as EventListener);
	});

	function handleError(error: string) {
		messages = [
			...messages,
			{
				text: `An error occurred: ${error}. Please try again.`,
				sender: 'ai',
				isError: true
			}
		];
	}

	async function handleNewMessage(text: string) {
		messages = [...messages, { text, sender: 'user', isError: false }];

		try {
			if (wsState.connected) {
				// Prefer WebSocket for real-time streaming if connected
				websocketStore.send({
					message: text,
					timestamp: new Date().toISOString()
				});
			} else {
				// Fallback to REST API if WebSocket is not available
				const response = await fetch(`${API_BASE_URL}/api/chat`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Authorization: `Bearer ${localStorage.getItem('auth_token')}`
					},
					body: JSON.stringify({
						message: text,
						timestamp: new Date().toISOString()
					})
				});

				if (!response.ok) {
					throw new Error(`HTTP error! status: ${response.status}`);
				}

				const data = await response.json();
				messages = [...messages, { text: data.message, sender: 'ai', isError: false }];
			}
		} catch (error) {
			console.error('Error sending message:', error);
			handleError('Failed to send message');
		}
	}
</script>

<NavBar />
<div class="flex h-screen flex-col items-center">
	<div class="w-3/4 h-full flex flex-col" in:slide={{ duration: 300, axis: 'y' }} out:slide={{ duration: 300, axis: 'y' }}>
		<ChatBox {messages} onMessage={handleNewMessage} />
	</div>
</div>

<style>
	/* Add any additional styles here */
</style>
