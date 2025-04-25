<script lang="ts">
	import '../app.css';
	import Navigation from '$lib/components/Navigation.svelte';

	let { children } = $props();
</script>

<div class="app min-h-screen flex flex-col">
    <Navigation />
    <main class="flex-grow">
        {@render children()}
    </main>
    <footer class="footer footer-center p-4 bg-base-300 text-base-content">
        <div>
            <p>Copyright © 2024 - Lyn AI Assistant</p>
        </div>
    </footer>
</div>
