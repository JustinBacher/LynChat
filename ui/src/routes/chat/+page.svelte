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
			// The message might already be parsed by the websocketStore
			const response = typeof event.detail === 'string' ? JSON.parse(event.detail) : event.detail;

			// Log the response for debugging
			console.log('WebSocket response received:', response);

			// Check for errors
			if (response.error) {
				handleError(typeof response.error === 'string' ? response.error : JSON.stringify(response.error));
				return;
			}

			// Extract content from the response
			let content = '';
			if (response.content) {
				content = response.content;
			} else if (response.message) {
				content = response.message;
			} else if (response.raw_response) {
				content = JSON.stringify(response.raw_response);
			} else {
				content = JSON.stringify(response);
			}

			// Add the message to the chat
			messages = [...messages, { text: content, sender: 'ai', isError: false }];
		} catch (error) {
			console.error('Error handling WebSocket message:', error);
			console.error('Raw event detail:', event.detail);
			handleError('Failed to parse response');
		}
	}

	onMount(() => {
		// Connect to WebSocket
		websocketStore.connect();

		// Add event listener for WebSocket messages
		if (typeof window !== 'undefined') {
			window.addEventListener('ws-message', handleWebSocketMessage as EventListener);
		}

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
		// Only remove event listener if we're in a browser environment
		if (typeof window !== 'undefined') {
			window.removeEventListener('ws-message', handleWebSocketMessage as EventListener);
		}
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
						Authorization: `Bearer ${typeof localStorage !== 'undefined' ? localStorage.getItem('auth_token') || '' : ''}`
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
