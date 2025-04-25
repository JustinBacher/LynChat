<script context="module" lang="ts">
	// Define the message structure
	export interface Message {
		text: string;
		sender: 'user' | 'ai';
		isError: boolean;
	}
</script>

<script lang="ts">
	// Props for messages and disabled state
	export let messages: Message[] = [];
	export let disabled: boolean = false;
	// Callback prop for handling messages
	export let onMessage: (text: string) => void = () => {};

	let textareaElement: HTMLTextAreaElement;
	let newMessageText: string = ''; // To hold the text being typed in the input

	// Function to handle sending a message
	function handleSendMessage() {
		if (newMessageText.trim()) {
			// Call the callback prop directly
			onMessage(newMessageText);
			newMessageText = ''; // Clear the input after sending
		}
	}
</script>

<div class="flex h-full flex-col">
	<div class="flex-1 space-y-4 overflow-y-auto p-4 mb-auto">
		{#each messages as message}
			<!-- DaisyUI chat bubble -->
			<div class="chat {message.sender === 'user' ? 'chat-end' : 'chat-start'}">
				<div
					class="chat-bubble {disabled
						? 'chat-bubble-neutral'
						: message.sender === 'user'
							? 'chat-bubble-primary'
							: 'chat-bubble-secondary'}"
				>
					{message.text}
				</div>
			</div>
		{/each}
	</div>

	<div class="flex p-4 mt-auto sticky bottom-0 bg-base-100">
		<textarea
			bind:this={textareaElement}
			bind:value={newMessageText}
			class="textarea textarea-lg textarea-secondary w-full"
			placeholder="Type your message here..."
		></textarea>
		<button class="btn btn-primary ml-2" on:click={handleSendMessage} aria-label="Send">
			<i class="fa-solid fa-paper-plane"></i>
		</button>
	</div>
</div>
