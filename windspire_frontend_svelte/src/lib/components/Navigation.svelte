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
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="flex justify-between h-16">
			<!-- Logo and brand -->
			<div class="flex items-center">
				<div class="flex-shrink-0 flex items-center">
					<!-- Windspire Logo -->
					<svg class="h-8 w-8 text-blue-600" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
					</svg>
					<span class="ml-2 text-xl font-bold text-gray-900">Windspire</span>
				</div>
			</div>

			<!-- Desktop Navigation -->
			<div class="hidden md:flex items-center space-x-8">
				{#each navigationItems as item}
					<button
						onclick={() => handleNavigation(item.href)}
						class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200"
						class:text-blue-600={$page.url.pathname === item.href}
						class:font-semibold={$page.url.pathname === item.href}
					>
						{item.name}
					</button>
				{/each}
			</div>

			<!-- User menu / Login -->
			<div class="hidden md:flex items-center space-x-4">
				{#if user}
					<div class="relative user-menu-container">
						<button
							onclick={toggleUserMenu}
							class="flex items-center space-x-3 text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 p-2 hover:bg-gray-50 transition-colors duration-200"
						>
							{#if user.picture}
								<img class="h-8 w-8 rounded-full" src={user.picture} alt={user.name} />
							{:else}
								<div class="h-8 w-8 rounded-full bg-blue-600 flex items-center justify-center">
									<span class="text-sm font-medium text-white">{user.name.charAt(0).toUpperCase()}</span>
								</div>
							{/if}
							<span class="text-gray-700 font-medium">{user.name}</span>
							<svg class="h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
							</svg>
						</button>

						{#if userMenuOpen}
							<div class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg py-1 z-50 border border-gray-200">
								<div class="px-4 py-2 border-b border-gray-100">
									<p class="text-sm font-medium text-gray-900">{user.name}</p>
									<p class="text-sm text-gray-500">{user.email}</p>
								</div>
								<button
									onclick={handleLogout}
									class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 transition-colors duration-200"
								>
									Sign out
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<button
						onclick={onLogin}
						class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200"
					>
						Login
					</button>
				{/if}
			</div>

			<!-- Mobile menu button -->
			<div class="md:hidden flex items-center">
				<button
					onclick={toggleMobileMenu}
					class="text-gray-700 hover:text-blue-600 focus:outline-none focus:text-blue-600 p-2"
				>
					<span class="sr-only">Open main menu</span>
					{#if !mobileMenuOpen}
						<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
						</svg>
					{:else}
						<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					{/if}
				</button>
			</div>
		</div>
	</div>

	<!-- Mobile menu -->
	{#if mobileMenuOpen}
		<div class="md:hidden">
			<div class="px-2 pt-2 pb-3 space-y-1 sm:px-3 bg-gray-50">
				{#each navigationItems as item}
					<button
						onclick={() => handleNavigation(item.href)}
						class="block w-full text-left text-gray-700 hover:text-blue-600 hover:bg-gray-100 px-3 py-2 rounded-md text-base font-medium transition-colors duration-200"
						class:text-blue-600={$page.url.pathname === item.href}
						class:bg-blue-50={$page.url.pathname === item.href}
					>
						{item.name}
					</button>
				{/each}
				
				<div class="border-t border-gray-200 pt-4">
					{#if user}
						<div class="flex items-center px-3 py-2 space-x-3">
							{#if user.picture}
								<img class="h-10 w-10 rounded-full" src={user.picture} alt={user.name} />
							{:else}
								<div class="h-10 w-10 rounded-full bg-blue-600 flex items-center justify-center">
									<span class="text-lg font-medium text-white">{user.name.charAt(0).toUpperCase()}</span>
								</div>
							{/if}
							<div>
								<p class="text-base font-medium text-gray-900">{user.name}</p>
								<p class="text-sm text-gray-500">{user.email}</p>
							</div>
						</div>
						<button
							onclick={handleLogout}
							class="block w-full text-left text-red-600 hover:bg-red-50 px-3 py-2 rounded-md text-base font-medium transition-colors duration-200"
						>
							Sign out
						</button>
					{:else}
						<button
							onclick={onLogin}
							class="block w-full text-left bg-blue-600 hover:bg-blue-700 text-white px-3 py-2 rounded-md text-base font-medium transition-colors duration-200"
						>
							Login
						</button>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</nav>