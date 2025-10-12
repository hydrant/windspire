<script lang="ts">
	import { onMount } from 'svelte';
	import { usersApi, type UserProfile } from '../api';

	console.log('UserProfile component is loading...');
	console.log('UserProfile component script executed!');

	// Get userId from route params (assuming it's passed as a prop by the router)
	let userId = $state<string>('');
	let userProfile = $state<UserProfile | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Function to extract userId from URL
	function getUserIdFromUrl(): string {
		const path = window.location.pathname;
		console.log('Current path:', path);
		const matches = path.match(/\/users\/([^/]+)/);
		console.log('Regex matches:', matches);
		return matches ? matches[1] : '';
	}

	async function loadUserProfile() {
		if (!userId) {
			error = 'No user ID provided';
			loading = false;
			return;
		}

		console.log('Loading user profile for userId:', userId);

		try {
			loading = true;
			error = null;
			userProfile = await usersApi.getUserProfile(userId);
			console.log('User profile loaded:', $state.snapshot(userProfile));
		} catch (err) {
			console.error('Failed to load user profile:', err);
			error = err instanceof Error ? err.message : 'Failed to load user profile';
		} finally {
			loading = false;
		}
	}

	onMount(async () => {
		userId = getUserIdFromUrl();
		console.log('Extracted userId from URL:', userId);
		await loadUserProfile();
	});
</script>

<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div
				class="h-8 w-8 animate-spin rounded-full border-4 border-blue-500 border-t-transparent"
			></div>
			<span class="ml-2 text-gray-600">Loading user profile...</span>
		</div>
	{:else if error}
		<div class="rounded-md bg-red-50 p-4">
			<div class="flex">
				<svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
						clip-rule="evenodd"
					></path>
				</svg>
				<div class="ml-3">
					<p class="text-sm text-red-800">{error}</p>
				</div>
			</div>
		</div>
	{:else if userProfile}
		<!-- User Profile Header -->
		<div class="mb-8 rounded-lg bg-white p-6 shadow-sm">
			<div class="flex items-center space-x-4">
				{#if userProfile.user.avatarUrl}
					<img
						src={userProfile.user.avatarUrl}
						alt="{userProfile.user.firstName} {userProfile.user.lastName}"
						class="h-16 w-16 rounded-full object-cover"
					/>
				{:else}
					<div
						class="flex h-16 w-16 items-center justify-center rounded-full bg-blue-500 text-white"
					>
						<span class="text-xl font-semibold">
							{userProfile.user.firstName?.charAt(0)}{userProfile.user.lastName?.charAt(0)}
						</span>
					</div>
				{/if}
				<div>
					<h1 class="text-3xl font-bold text-gray-900">
						{userProfile.user.firstName}
						{userProfile.user.lastName}
					</h1>
					<p class="text-gray-600">{userProfile.user.email}</p>
					{#if userProfile.user.isoName}
						<p class="text-sm text-gray-500">{userProfile.user.isoName}</p>
					{/if}
				</div>
			</div>
		</div>

		<!-- Boats Section -->
		<div class="rounded-lg bg-white shadow-sm">
			<div class="border-b border-gray-200 px-6 py-4">
				<h2 class="text-xl font-semibold text-gray-900">
					Boats ({userProfile.boat_count})
				</h2>
			</div>

			{#if userProfile.boats.length === 0}
				<div class="px-6 py-12 text-center">
					<svg
						class="mx-auto h-12 w-12 text-gray-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2M4 13h2m13-8V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2M4 13h2"
						></path>
					</svg>
					<h3 class="mt-2 text-sm font-medium text-gray-900">No boats</h3>
					<p class="mt-1 text-sm text-gray-500">This user doesn't own any boats yet.</p>
				</div>
			{:else}
				<div class="divide-y divide-gray-200">
					{#each userProfile.boats as boat (boat.id)}
						<div class="px-6 py-4">
							<div class="flex items-center justify-between">
								<div>
									<h3 class="text-lg font-medium text-gray-900">{boat.name}</h3>
									<div class="mt-1 flex space-x-4 text-sm text-gray-500">
										{#if boat.brand}
											<span>{boat.brand}</span>
										{/if}
										{#if boat.model}
											<span>{boat.model}</span>
										{/if}
										{#if boat.sailNumber}
											<span>Sail #{boat.sailNumber}</span>
										{/if}
									</div>
								</div>
								<div class="flex items-center space-x-2">
									{#if boat.countryId}
										<span
											class="inline-flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800"
										>
											{boat.countryId}
										</span>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>
