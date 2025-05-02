<script lang="ts">
	import { slide } from 'svelte/transition';
	import { Textarea, Button, Alert } from 'flowbite-svelte';
	import { PaperPlaneOutline, InfoCircleSolid } from 'flowbite-svelte-icons';

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

	// This is a placeholder component and should not be used directly
	// Use ChatBox.svelte instead which has full message handling functionality
</script>

<div class="flex-container flex h-screen flex-col items-start" in:slide={{ duration: 300, axis: 'y' }} out:slide={{ duration: 300, axis: 'y' }}>
	<Alert color="red" class="mb-4">
		<InfoCircleSolid slot="icon" class="h-4 w-4" />
		<span class="font-medium">Warning!</span> This is a placeholder component. Use ChatBox.svelte instead.
	</Alert>

	<div
		class="min-w-2xl grid w-full grid-cols-[1fr_auto] grid-rows-1 gap-4"
	>
		<div class="grid-item">
			<Textarea
				class="min-h-[3rem] resize-none transition-height duration-100 ease-in"
				on:input={autoResize}
				rows={1}
				placeholder="This component has no message handling functionality"
			/>
		</div>
		<div class="grid-item self-end">
			<Button size="lg" color="primary" aria-label="Send">
				<PaperPlaneOutline class="w-5 h-5" />
			</Button>
		</div>
	</div>
</div>


