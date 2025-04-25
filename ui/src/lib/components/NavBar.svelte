<script>
	import { onMount } from 'svelte';
	import { themeChange } from 'theme-change';

	// NOTE: the element that is using one of the theme attributes must be in the DOM on mount
	onMount(() => {
		themeChange(false);
	});
</script>

<div class="navbar bg-base-100 shadow-sm">
	<div class="navbar-start">
		<div class="dropdown">
			<div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h8m-8 6h16"
					/>
				</svg>
			</div>
			<ul
				tabindex="-1"
				class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
			>
				<li><a href="/pricing">Services</a></li>
				<li>
					<a>Parent</a>
					<ul class="p-2">
						<li><a>Submenu 1</a></li>
						<li><a>Submenu 2</a></li>
					</ul>
				</li>
				<li><a href="/about">About</a></li>
			</ul>
		</div>
		<a href="/">
			<i class="fa-solid fa-brain text-2xl"></i>
			<span class="pl-4 text-2xl">Lyn</span>
		</a>
	</div>
	<div class="navbar-center hidden lg:flex">
		<ul class="menu menu-horizontal px-1">
			<li><a href="/pricing">Pricing</a></li>
			<li>
				<details>
					<summary>Parent</summary>
					<ul class="p-2">
						<li><a>Submenu 1</a></li>
						<li><a>Submenu 2</a></li>
					</ul>
				</details>
			</li>
			<li><a>Item 3</a></li>
		</ul>
	</div>
	<div class="navbar-end">
		<div>
			<label class="swap swap-rotate p-4">
				<!-- this hidden checkbox controls the state -->
				<input type="checkbox" class="theme-controller" id="theme-toggle" value="sunset" data-toggle-theme="dark,light" style="display: none;" />

				<!-- sun icon -->
				<i class="fa-solid fa-sun swap-off fill-current"></i>

				<!-- moon icon -->
				<i class="fa-solid fa-moon swap-on fill-current"></i>
			</label>
			<a href="/login" class="btn"><i class="fa-solid fa-arrow-right-to-bracket"></i>Sign in</a>
		</div>
	</div>
</div>
