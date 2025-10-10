<script lang="ts">
	import { userStore } from '../stores/user';
	import { config } from '../config';
	import { route } from '@mateothegreat/svelte5-router';
	import { onMount } from 'svelte';

	const { openLoginModal }: { openLoginModal: () => void } = $props();

	let isUserMenuOpen = $state(false);

	function logout() {
		localStorage.removeItem('windspire_token');
		userStore.set(null);
		// Force navigation to home page
		window.location.href = window.location.origin + '/';
		isUserMenuOpen = false;
	}

	function toggleUserMenu() {
		isUserMenuOpen = !isUserMenuOpen;
	}

	function closeUserMenu() {
		isUserMenuOpen = false;
	}

	// Close menu when clicking outside
	onMount(() => {
		function handleClickOutside(event: MouseEvent) {
			if (isUserMenuOpen) {
				const target = event.target as HTMLElement;
				if (!target.closest('.user-menu-container')) {
					closeUserMenu();
				}
			}
		}

		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});
</script>

<nav class="border-b border-gray-200 bg-white shadow-sm">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="flex h-16 items-center justify-between">
			<!-- Logo -->
			<div class="flex items-center">
				<a href="/" use:route class="flex items-center">
					<svg class="mr-2 h-8 w-8 text-blue-600" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
					</svg>
					<span class="text-xl font-bold text-gray-900">{config.APP_NAME}</span>
				</a>
			</div>

			<!-- Navigation Links -->
			<div class="hidden items-center space-x-8 md:flex">
				<a
					href="/"
					use:route
					class="font-medium text-gray-700 transition-colors duration-200 hover:text-blue-600"
				>
					Home
				</a>
				{#if $userStore}
					<a
						href="/boats"
						use:route
						class="font-medium text-gray-700 transition-colors duration-200 hover:text-blue-600"
					>
						Boats
					</a>
				{/if}
			</div>

			<!-- User Menu -->
			<div class="flex items-center space-x-4">
				{#if $userStore}
					<div class="user-menu-container relative">
						<button
							onclick={toggleUserMenu}
							class="flex items-center space-x-3 rounded-lg p-2 transition-colors duration-200 hover:bg-gray-50"
						>
							{#if $userStore.picture}
								<img src={$userStore.picture} alt={$userStore.name} class="h-8 w-8 rounded-full" />
							{:else}
								<div class="flex h-8 w-8 items-center justify-center rounded-full bg-blue-600">
									<span class="text-sm font-medium text-white">
										{$userStore.name.charAt(0).toUpperCase()}
									</span>
								</div>
							{/if}

							<div class="hidden text-left md:block">
								<p class="text-sm font-medium text-gray-900">{$userStore.name}</p>
								<p class="text-xs text-gray-600">{$userStore.email}</p>
							</div>

							<!-- Dropdown arrow -->
							<svg
								class="h-4 w-4 text-gray-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								></path>
							</svg>
						</button>

						<!-- Dropdown Menu -->
						{#if isUserMenuOpen}
							<div
								class="absolute right-0 top-full z-50 mt-2 w-48 rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5"
							>
								<a
									href="/users/{$userStore.id}"
									use:route
									onclick={() => closeUserMenu()}
									class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
								>
									<svg
										class="mr-3 inline h-4 w-4"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
										></path>
									</svg>
									My Profile
								</a>
								<button
									onclick={logout}
									class="block w-full px-4 py-2 text-left text-sm text-gray-700 hover:bg-gray-100"
								>
									<svg
										class="mr-3 inline h-4 w-4"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
										></path>
									</svg>
									Sign Out
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<button
						onclick={openLoginModal}
						class="rounded-lg bg-blue-600 px-4 py-2 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
					>
						Sign In
					</button>
				{/if}
			</div>
		</div>
	</div>
</nav>
