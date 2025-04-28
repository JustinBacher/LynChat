<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { slide } from 'svelte/transition';
	import ChatBox from '$lib/components/ChatBox.svelte';
	import type { Message } from '$lib/components/ChatBox.svelte';
	import { initialMessageStore } from '$lib/stores/chatStore';

	// Get the initial message from the store
	let initialMessage = '';
	const unsubscribeInitialMessage = initialMessageStore.subscribe((value: string) => {
		initialMessage = value;
	});

	// Initialize messages array with a welcome message
	let messages: Message[] = [
		{ text: 'Hello! How can I help you today?', sender: 'ai', isError: false }
	];

	// Process initial message if it exists
	onMount(() => {
		if (initialMessage.trim()) {
			// Add a slight delay to ensure the transition completes first
			setTimeout(() => {
				// Add the initial message to the messages array
				messages = [...messages, { text: initialMessage, sender: 'user', isError: false }];

				// Clear the store after using it
				initialMessageStore.set('');
			}, 500);
		}
	});

	onDestroy(() => {
		// Clean up subscriptions
		unsubscribeInitialMessage();
	});
</script>

<div class="flex h-screen flex-col items-center">
	<div class="w-3/4 h-full flex flex-col" in:slide={{ duration: 300, axis: 'y' }} out:slide={{ duration: 300, axis: 'y' }}>
		<ChatBox {messages} />
	</div>
</div>

<style>
	/* Add any additional styles here */
</style>
