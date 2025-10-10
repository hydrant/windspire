<script lang="ts">
	import { userStore } from '../stores/user';
	import { config } from '../config';
	import { route } from '@mateothegreat/svelte5-router';

	const { openLoginModal }: { openLoginModal: () => void } = $props();

	function logout() {
		localStorage.removeItem('windspire_token');
		userStore.set(null);
		window.dispatchEvent(new CustomEvent('auth-changed'));
		// Redirect to home
		window.location.hash = '/';
	}
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
				<a
					href="/boats"
					use:route
					class="font-medium text-gray-700 transition-colors duration-200 hover:text-blue-600"
				>
					Boats
				</a>
			</div>

			<!-- User Menu -->
			<div class="flex items-center space-x-4">
				{#if $userStore}
					<div class="flex items-center space-x-3">
						{#if $userStore.picture}
							<img src={$userStore.picture} alt={$userStore.name} class="h-8 w-8 rounded-full" />
						{:else}
							<div class="flex h-8 w-8 items-center justify-center rounded-full bg-blue-600">
								<span class="text-sm font-medium text-white">
									{$userStore.name.charAt(0).toUpperCase()}
								</span>
							</div>
						{/if}

						<div class="hidden md:block">
							<p class="text-sm font-medium text-gray-900">{$userStore.name}</p>
							<p class="text-xs text-gray-600">{$userStore.email}</p>
						</div>

						<button
							onclick={logout}
							class="font-medium text-gray-700 transition-colors duration-200 hover:text-red-600"
						>
							Sign Out
						</button>
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
