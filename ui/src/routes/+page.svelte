<script lang="ts">
	import NavBar from '$lib/components/NavBar.svelte';
	import StartChat from '$lib/components/StartChat.svelte';
	import { page } from '$app/state';
</script>

<NavBar />
<div class="flex h-screen flex-col items-center justify-center">
	<div class="p-30 flex w-full flex-col items-center justify-center text-6xl">
		<h1 class="animate-fade-up animate-once animate-duration-1500 pt-10 text-center text-4xl font-bold">
			Hi Justin!
		</h1>
	</div>
	<StartChat />
</div>

<style lang="less">
</style>
