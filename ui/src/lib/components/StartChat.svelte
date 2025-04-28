<script lang="ts">
	// Import onDestroy lifecycle hook
	import { onDestroy, onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { slide } from 'svelte/transition';
	import createTypingPlaceholderStore from '$lib/stores/typingPlaceholderStore';
	import { initialMessageStore } from '$lib/stores/chatStore';
	import { Textarea, Button, Badge } from 'flowbite-svelte';
	import { PaperPlaneOutline } from 'flowbite-svelte-icons';

	const placeholderStrings: string[] = [
		"I've never told anyone this, but...",
		"What do I do if I'm feeling overwhelmed?",
		'How do I talk to someone about something personal?',
		'Can I say this here without it being saved forever?',
		"I need to write something out, even if I don't send it.",
		"Help me make sense of what I'm feeling.",
		"Is it okay that I don't have it all figured out?",
		'Why does this keep bothering me?',
		'What if I just need to vent for a second?',
		'Who sees what I type here?',
		"Can I erase this after we're done?",
		'Make sure this stays between us.',
		'Can you help me redact sensitive info before I share?'
	];

	// Example prompts that users can click on
	const examplePrompts: string[] = [
		'Tell me what you can do',
		'Tell me a joke',
		'Give me a cookie recipe',
		'When will I figure everything out',
		'How can you help me today',
		'What makes you different',
		'Write a short poem',
		'Explain quantum computing',
		'Give me a fun fact',
		'Help me plan my day'
	];

	// Randomly select 4 example prompts to display
	let selectedPrompts: string[] = [];

	function selectRandomPrompts() {
		// Shuffle the array and take the first 4 elements
		const shuffled = [...examplePrompts].sort(() => 0.5 - Math.random());
		selectedPrompts = shuffled.slice(0, 4);
	}

	// Handle clicking on an example prompt
	function handlePromptClick(prompt: string) {
		messageText = prompt;
		// Also resize the textarea
		setTimeout(() => {
			const textarea = document.querySelector('textarea');
			if (textarea) {
				textarea.style.height = 'auto';
				textarea.style.height = textarea.scrollHeight + 'px';
			}
		}, 0);
	}

	const typingSpeed: number = 70;
	const stallDuration: number = 2000;

	const typingStore = createTypingPlaceholderStore(placeholderStrings, typingSpeed, stallDuration);

	let messageText: string = '';

	// With Flowbite Svelte, we'll use a different approach for auto-resizing
	function autoResize(e: Event) {
		const textarea = e.target as HTMLTextAreaElement;
		if (textarea) {
			// Reset height to allow shrinking
			textarea.style.height = 'auto';
			// Set the height to match the scroll height
			textarea.style.height = textarea.scrollHeight + 'px';
		}
	}

	function handleSend() {
		if (messageText.trim()) {
			// Store the message in the store
			initialMessageStore.set(messageText);
			// Navigate to the chat page
			goto('/chat');
		}
	}

	// Handle keyboard events for the textarea
	function handleKeydown(e: KeyboardEvent) {
		// Send message on Enter key (without Shift key)
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault(); // Prevent default behavior (new line)
			handleSend();
		}
	}

	onMount(() => {
		// Initialize textarea height on mount
		const textarea = document.querySelector('textarea');
		if (textarea) {
			textarea.style.height = 'auto';
			textarea.style.height = textarea.scrollHeight + 'px';
		}

		// Select random prompts on mount
		selectRandomPrompts();
	});

	onDestroy(() => {
		typingStore.stop(); // Call the cleanup method on the store object
	});
</script>

<div class="flex-container flex h-screen flex-col items-start" in:slide={{ duration: 300, axis: 'y' }} out:slide={{ duration: 300, axis: 'y' }}>
	<div></div>
	<div
		class="min-w-2xl w-full flex flex-col gap-4"
	>
		<!-- Example prompts -->
		<div class="flex justify-center gap-2 mb-6 w-1/2 mx-auto">
			{#each selectedPrompts as prompt}
				<Badge
					rounded
					color="primary"
					class="cursor-pointer hover:bg-primary-200 dark:hover:bg-primary-600 transition-colors text-base py-1.5 px-3 flex-shrink-0 h-auto flex items-start text-left break-words max-w-[120px] whitespace-normal"
					on:click={() => handlePromptClick(prompt)}
				>
					<div class="w-full">
						{prompt}
					</div>
				</Badge>
			{/each}
		</div>

		<!-- Input area -->
		<div class="grid w-full grid-cols-[1fr_auto] grid-rows-1 gap-4">
			<div class="grid-item">
				<Textarea
					bind:value={messageText}
					class="min-h-[3rem] resize-none transition-height duration-100 ease-in"
					placeholder={$typingStore}
					on:input={autoResize}
					on:keydown={handleKeydown}
					rows={1}
				/>
			</div>
			<div class="grid-item self-end">
				<Button size="lg" color="primary" aria-label="Send" on:click={handleSend}>
					<PaperPlaneOutline class="w-5 h-5" />
				</Button>
			</div>
		</div>
	</div>
</div>


