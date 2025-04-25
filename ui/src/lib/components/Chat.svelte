<script lang="ts">
	import {slide} from 'svelte/transition';

	let textareaElement: HTMLTextAreaElement;

	function autoResize() {
		if (textareaElement) {
			// Reset height to allow shrinking
			textareaElement.style.height = 'auto';
			// Set the height to match the scroll height
			textareaElement.style.height = textareaElement.scrollHeight + 'px';
		}
	}
</script>

<div class="flex-container flex h-screen flex-col items-start" in:slide={{ duration: 300, axis: 'y' }} out:slide={{ duration: 300, axis: 'y' }}>
	<div></div>
	<div
		class="min-w-2xl grid w-full grid-cols-[1fr_auto] grid-rows-1 gap-4"
	>
		<div class="grid-item">
			<textarea
				bind:this={textareaElement}
				class="textarea textarea-lg textarea-autosize textarea-secondary w-full"
				on:input={autoResize}
			></textarea>
		</div>
		<div class="grid-item self-end">
			<button class="btn btn-primary btn-lg" aria-label="Send">
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
