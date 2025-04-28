<script lang="ts">
	import NavBar from '$lib/components/NavBar.svelte';
	import '../app.css';
	import { Footer } from 'flowbite-svelte';

	let { children } = $props();
</script>

<div class="app min-h-screen flex flex-col">
    <NavBar />
    <main class="flex-grow">
        {@render children()}
    </main>
    <Footer footerType="logo">
        <p>Copyright © 2024 - Lyn AI Assistant</p>
    </Footer>
</div>
