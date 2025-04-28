<script context="module" lang="ts">
	// Define the message structure
	export interface Message {
		text: string;
		sender: 'user' | 'ai';
		isError: boolean;
	}
</script>

<script lang="ts">
	import { Textarea, Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { PaperPlaneOutline, BrainOutline, ChevronDownOutline } from 'flowbite-svelte-icons';
	import { onMount, onDestroy } from 'svelte';
	import { chatStore, type ChatOptions } from '$lib/stores/chatStore';

	// Props for messages and disabled state
	export let messages: Message[] = [];
	export let disabled: boolean = false;
	// Callback prop for handling messages (legacy support)
	export let onMessage: (text: string) => void = () => {};

	let newMessageText: string = ''; // To hold the text being typed in the input
	let reasoningEnabled: boolean = false; // Toggle for reasoning mode
	let selectedModel: string = 'Default Model'; // Currently selected model
	let isStreaming: boolean = false; // Track if we're currently streaming a response

	// Subscribe to the chat store state
	const unsubscribe = chatStore.subscribe(state => {
		isStreaming = state.isStreaming;
		if (state.error) {
			console.error('Chat store error:', state.error);
		}
	});

	// Available models
	const models = [
		{ id: 'model1', name: 'Default Model' },
		{ id: 'model2', name: 'GPT-3.5 Turbo' },
		{ id: 'model3', name: 'GPT-4' },
		{ id: 'model4', name: 'Claude 3 Opus' },
		{ id: 'model5', name: 'Gemma 7B' }
	];

	// Function to handle sending a message
	async function handleSendMessage() {
		if (newMessageText.trim()) {
			const messageText = newMessageText;
			newMessageText = ''; // Clear the input immediately

			// Add user message to the messages array
			messages = [...messages, { text: messageText, sender: 'user', isError: false }];

			// Add a "thinking" message from the AI
			const thinkingIndex = messages.length;
			messages = [...messages, { text: 'Thinking...', sender: 'ai', isError: false }];

			try {
				// Prepare options for the chat request
				const options: ChatOptions = {
					model: selectedModel !== 'Default Model' ? selectedModel : undefined,
					reasoningEnabled
				};

				// Use the chat store to send the message and get a streaming response
				const finalResponse = await chatStore.sendMessage(messageText, options);

				// Replace the "thinking" message with the final response
				messages[thinkingIndex] = {
					text: finalResponse,
					sender: 'ai',
					isError: false
				};
				messages = [...messages]; // Trigger reactivity
			} catch (err) {
				const error = err as Error;
				console.error('Error sending message:', error);

				// Replace the "thinking" message with an error
				messages[thinkingIndex] = {
					text: `Error: ${error.message || 'Failed to get response'}`,
					sender: 'ai',
					isError: true
				};
				messages = [...messages]; // Trigger reactivity
			}
		}
	}

	// Handle key press events in the textarea
	function handleKeyDown(event: KeyboardEvent) {
		// Send message on Enter key (without Shift key)
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault(); // Prevent default behavior (new line)
			handleSendMessage();
		}
	}

	// Toggle reasoning mode
	function toggleReasoning() {
		reasoningEnabled = !reasoningEnabled;
	}

	// Select a model
	function selectModel(modelName: string) {
		selectedModel = modelName;
	}

	// Set up event listeners for streaming updates
	function setupStreamEventListeners() {
		if (typeof window !== 'undefined') {
			// Listen for chunk events to update the UI in real-time
			window.addEventListener('chat-stream-chunk', handleStreamChunk as EventListener);
			window.addEventListener('chat-stream-error', handleStreamError as EventListener);
		}
	}

	// Handle streaming chunks
	function handleStreamChunk(event: CustomEvent) {
		// Find the "thinking" message (should be the last AI message)
		const aiMessageIndex = messages.findLastIndex(m => m.sender === 'ai');
		if (aiMessageIndex >= 0) {
			// Update the message with the accumulated content
			messages[aiMessageIndex] = {
				...messages[aiMessageIndex],
				text: event.detail.accumulated_content
			};
			messages = [...messages]; // Trigger reactivity
		}
	}

	// Handle streaming errors
	function handleStreamError(event: CustomEvent) {
		console.error('Stream error:', event.detail);
		// Find the "thinking" message
		const aiMessageIndex = messages.findLastIndex(m => m.sender === 'ai');
		if (aiMessageIndex >= 0) {
			// Update the message with the error
			messages[aiMessageIndex] = {
				text: `Error: ${event.detail.error}`,
				sender: 'ai',
				isError: true
			};
			messages = [...messages]; // Trigger reactivity
		}
	}

	// Clean up event listeners
	function cleanupStreamEventListeners() {
		if (typeof window !== 'undefined') {
			window.removeEventListener('chat-stream-chunk', handleStreamChunk as EventListener);
			window.removeEventListener('chat-stream-error', handleStreamError as EventListener);
		}
	}

	onMount(() => {
		setupStreamEventListeners();
	});

	onDestroy(() => {
		unsubscribe();
		cleanupStreamEventListeners();
		// Cancel any ongoing requests when component is destroyed
		chatStore.cancelRequest();
	});
</script>

<div class="flex h-full flex-col">
	<div class="flex-1 space-y-4 overflow-y-auto p-4 mb-auto">
		{#each messages as message}
			<!-- Flowbite styled chat bubble -->
			<div class="flex items-start {message.sender === 'user' ? 'justify-end' : 'justify-start'} gap-2.5">
				{#if message.sender === 'ai'}
					<div class="flex flex-col w-full max-w-[320px] leading-1.5 p-4 border border-gray-200 dark:border-gray-700 bg-gray-100 dark:bg-gray-800 rounded-e-xl rounded-es-xl shadow-sm">
						<p class="text-sm font-normal py-2.5 text-gray-800 dark:text-gray-100">
							{message.text}
						</p>
					</div>
				{:else}
					<div class="flex flex-col w-full max-w-[320px] leading-1.5 p-4 border border-gray-200 dark:border-gray-700 {message.isError ? 'bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-100' : 'bg-prussian-100 dark:bg-prussian-900 text-prussian-800 dark:text-prussian-100'} rounded-s-xl rounded-ee-xl shadow-sm">
						<p class="text-sm font-normal py-2.5">
							{message.text}
						</p>
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<div class="flex flex-col p-4 pb-8 mt-auto sticky bottom-0 bg-white dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700">
		<!-- Textarea container with rounded corners and background -->
		<div class="w-full mb-4 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm">
			<Textarea
				bind:value={newMessageText}
				class="w-full bg-transparent border-0 focus:ring-0 p-0 resize-none text-gray-800 dark:text-gray-100"
				placeholder="Type your message here..."
				rows={2}
				disabled={disabled}
				on:keydown={handleKeyDown}
			/>
		</div>

		<!-- Buttons row -->
		<div class="flex justify-between items-center">
			<div class="flex space-x-2">
				<!-- Reasoning toggle button -->
				<Button
					color={reasoningEnabled ? "primary" : "alternative"}
					size="sm"
					on:click={toggleReasoning}
					disabled={disabled}
					class="transition-colors duration-200"
				>
					<BrainOutline class="w-4 h-4 mr-1" />
					{reasoningEnabled ? 'Reasoning On' : 'Reasoning Off'}
				</Button>

				<!-- Model selector dropdown -->
				<Button id="model-selector" color="alternative" size="sm" disabled={disabled} class="transition-colors duration-200">
					<span class="mr-1">{selectedModel}</span>
					<ChevronDownOutline class="w-4 h-4" />
				</Button>
				<Dropdown triggeredBy="#model-selector" class="z-10">
					{#each models as model}
						<DropdownItem on:click={() => selectModel(model.name)}>
							{model.name}
						</DropdownItem>
					{/each}
				</Dropdown>
			</div>

			<!-- Send button -->
			<Button color="primary" on:click={handleSendMessage} aria-label="Send" disabled={disabled} class="transition-colors duration-200">
				<PaperPlaneOutline class="w-5 h-5 mr-1" />
				Send
			</Button>
		</div>
	</div>
</div>
