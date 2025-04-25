<script lang="ts">
	// Import onDestroy lifecycle hook
	import { onDestroy, onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { slide } from 'svelte/transition';
	import createTypingPlaceholderStore from '$lib/stores/typingPlaceholderStore';
	import { initialMessageStore } from '$lib/stores/chatStore';

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
	const typingSpeed: number = 70;
	const stallDuration: number = 2000;

	const typingStore = createTypingPlaceholderStore(placeholderStrings, typingSpeed, stallDuration);

	let textareaElement: HTMLTextAreaElement;
	let messageText: string = '';

	function autoResize() {
		if (textareaElement) {
			// Reset height to allow shrinking
			textareaElement.style.height = 'auto';
			// Set the height to match the scroll height
			textareaElement.style.height = textareaElement.scrollHeight + 'px';
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

	onMount(() => {
		autoResize();
	});

	onDestroy(() => {
		typingStore.stop(); // Call the cleanup method on the store object
	});
</script>

<div class="flex-container flex h-screen flex-col items-start" in:slide={{ duration: 300, axis: 'y' }} out:slide={{ duration: 300, axis: 'y' }}>
	<div></div>
	<div
		class="min-w-2xl grid w-full grid-cols-[1fr_auto] grid-rows-1 gap-4"
	>
		<div class="grid-item">
			<textarea
				bind:this={textareaElement}
				bind:value={messageText}
				class="textarea textarea-lg textarea-autosize textarea-secondary w-full"
				placeholder={$typingStore}
				on:input={autoResize}
			></textarea>
		</div>
		<div class="grid-item self-end">
			<button class="btn btn-primary btn-lg" aria-label="Send" on:click={handleSend}>
				<i class="fa-solid fa-paper-plane"></i>
			</button>
		</div>
	</div>
</div>

<style lang="less">
	.textarea-autosize {
		height: 1rem;
		overflow: hidden; // Hide scrollbar
		resize: none; // Prevent manual resizing handle
		line-height: 1.5rem;
		transition: height 0.1s ease;
	}
</style>
