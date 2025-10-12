<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	interface Props {
		user?: { name: string; email: string; picture?: string } | null;
		onLogin?: () => void;
		onLogout?: () => void;
	}

	const { user = null, onLogin, onLogout }: Props = $props();

	let mobileMenuOpen = $state(false);
	let userMenuOpen = $state(false);

	const navigationItems = [
		{ name: 'Home', href: '/', current: true },
		{ name: 'Boats', href: '/boats', current: false },
		{ name: 'About', href: '/about', current: false },
		{ name: 'Services', href: '/services', current: false },
		{ name: 'Contact', href: '/contact', current: false }
	];

	function handleNavigation(href: string) {
		goto(href);
		mobileMenuOpen = false;
	}

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}

	function toggleUserMenu() {
		userMenuOpen = !userMenuOpen;
	}

	function closeUserMenu() {
		userMenuOpen = false;
	}

	function handleLogout() {
		closeUserMenu();
		if (onLogout) onLogout();
	}

	// Close user menu when clicking outside
	onMount(() => {
		function handleClickOutside(event: MouseEvent) {
			const target = event.target as Element;
			if (userMenuOpen && target && !target.closest('.user-menu-container')) {
				closeUserMenu();
			}
		}

		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});
</script>

<nav class="bg-white shadow-lg">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 justify-between">
			<!-- Logo and brand -->
			<div class="flex items-center">
				<div class="flex flex-shrink-0 items-center">
					<!-- Windspire Logo -->
					<svg class="h-8 w-8 text-blue-600" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
					</svg>
					<span class="ml-2 text-xl font-bold text-gray-900">Windspire</span>
				</div>
			</div>

			<!-- Desktop Navigation -->
			<div class="hidden items-center space-x-8 md:flex">
				{#each navigationItems as item (item.href)}
					<button
						onclick={() => handleNavigation(item.href)}
						class="rounded-md px-3 py-2 text-sm font-medium text-gray-700 transition-colors duration-200 hover:text-blue-600"
						class:text-blue-600={$page.url.pathname === item.href}
						class:font-semibold={$page.url.pathname === item.href}
					>
						{item.name}
					</button>
				{/each}
			</div>

			<!-- User menu / Login -->
			<div class="hidden items-center space-x-4 md:flex">
				{#if user}
					<div class="user-menu-container relative">
						<button
							onclick={toggleUserMenu}
							class="flex items-center space-x-3 rounded-full p-2 text-sm transition-colors duration-200 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
						>
							{#if user.picture}
								<img class="h-8 w-8 rounded-full" src={user.picture} alt={user.name} />
							{:else}
								<div class="flex h-8 w-8 items-center justify-center rounded-full bg-blue-600">
									<span class="text-sm font-medium text-white"
										>{user.name.charAt(0).toUpperCase()}</span
									>
								</div>
							{/if}
							<span class="font-medium text-gray-700">{user.name}</span>
							<svg
								class="h-4 w-4 text-gray-400"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								/>
							</svg>
						</button>

						{#if userMenuOpen}
							<div
								class="absolute right-0 z-50 mt-2 w-48 rounded-md border border-gray-200 bg-white py-1 shadow-lg"
							>
								<div class="border-b border-gray-100 px-4 py-2">
									<p class="text-sm font-medium text-gray-900">{user.name}</p>
									<p class="text-sm text-gray-500">{user.email}</p>
								</div>
								<button
									onclick={handleLogout}
									class="block w-full px-4 py-2 text-left text-sm text-gray-700 transition-colors duration-200 hover:bg-gray-100"
								>
									Sign out
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<button
						onclick={onLogin}
						class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors duration-200 hover:bg-blue-700"
					>
						Login
					</button>
				{/if}
			</div>

			<!-- Mobile menu button -->
			<div class="flex items-center md:hidden">
				<button
					onclick={toggleMobileMenu}
					class="p-2 text-gray-700 hover:text-blue-600 focus:text-blue-600 focus:outline-none"
				>
					<span class="sr-only">Open main menu</span>
					{#if !mobileMenuOpen}
						<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 6h16M4 12h16M4 18h16"
							/>
						</svg>
					{:else}
						<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					{/if}
				</button>
			</div>
		</div>
	</div>

	<!-- Mobile menu -->
	{#if mobileMenuOpen}
		<div class="md:hidden">
			<div class="space-y-1 bg-gray-50 px-2 pb-3 pt-2 sm:px-3">
				{#each navigationItems as item (item.href)}
					<button
						onclick={() => handleNavigation(item.href)}
						class="block w-full rounded-md px-3 py-2 text-left text-base font-medium text-gray-700 transition-colors duration-200 hover:bg-gray-100 hover:text-blue-600"
						class:text-blue-600={$page.url.pathname === item.href}
						class:bg-blue-50={$page.url.pathname === item.href}
					>
						{item.name}
					</button>
				{/each}

				<div class="border-t border-gray-200 pt-4">
					{#if user}
						<div class="flex items-center space-x-3 px-3 py-2">
							{#if user.picture}
								<img class="h-10 w-10 rounded-full" src={user.picture} alt={user.name} />
							{:else}
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-blue-600">
									<span class="text-lg font-medium text-white"
										>{user.name.charAt(0).toUpperCase()}</span
									>
								</div>
							{/if}
							<div>
								<p class="text-base font-medium text-gray-900">{user.name}</p>
								<p class="text-sm text-gray-500">{user.email}</p>
							</div>
						</div>
						<button
							onclick={handleLogout}
							class="block w-full rounded-md px-3 py-2 text-left text-base font-medium text-red-600 transition-colors duration-200 hover:bg-red-50"
						>
							Sign out
						</button>
					{:else}
						<button
							onclick={onLogin}
							class="block w-full rounded-md bg-blue-600 px-3 py-2 text-left text-base font-medium text-white transition-colors duration-200 hover:bg-blue-700"
						>
							Login
						</button>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</nav>
