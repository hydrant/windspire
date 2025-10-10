<script lang="ts">
	import { onMount } from 'svelte';
	import { boatsApi, type Boat } from '../api';
	import { userStore } from '../stores/user';

	let boats = $state<Boat[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			const result = await boatsApi.getBoats();
			// Handle the paginated result
			boats = Array.isArray(result) ? result : result.data || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load boats';
		} finally {
			loading = false;
		}
	});
</script>

<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-8 flex items-center justify-between">
		<h1 class="text-3xl font-bold text-gray-900">Boats</h1>
		{#if $userStore}
			<button
				class="rounded-lg bg-blue-600 px-4 py-2 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
			>
				Add Boat
			</button>
		{/if}
	</div>

	{#if loading}
		<div class="flex items-center justify-center py-16">
			<div class="spinner"></div>
			<span class="ml-2 text-gray-600">Loading boats...</span>
		</div>
	{:else if error}
		<div class="rounded-lg border border-red-200 bg-red-50 p-4">
			<div class="flex">
				<svg class="h-5 w-5 text-red-400" fill="currentColor" viewBox="0 0 20 20">
					<path
						fill-rule="evenodd"
						d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
						clip-rule="evenodd"
					/>
				</svg>
				<div class="ml-3">
					<h3 class="text-sm font-medium text-red-800">Error</h3>
					<p class="mt-1 text-sm text-red-700">{error}</p>
				</div>
			</div>
		</div>
	{:else if boats.length === 0}
		<div class="py-16 text-center">
			<svg
				class="mx-auto mb-4 h-16 w-16 text-gray-400"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
				/>
			</svg>
			<h3 class="mb-2 text-lg font-medium text-gray-900">No boats found</h3>
			<p class="mb-4 text-gray-600">Get started by adding your first boat to the system.</p>
			{#if $userStore}
				<button
					class="rounded-lg bg-blue-600 px-4 py-2 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
				>
					Add Your First Boat
				</button>
			{/if}
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each boats as boat}
				<div
					class="overflow-hidden rounded-lg bg-white shadow-md transition-shadow duration-200 hover:shadow-lg"
				>
					<div class="p-6">
						<h3 class="mb-2 text-lg font-semibold text-gray-900">{boat.name}</h3>
						<p class="mb-4 text-gray-600">{boat.model || 'Unknown Model'}</p>

						<div class="space-y-2 text-sm text-gray-600">
							{#if boat.brand}
								<div class="flex items-center">
									<span class="font-medium">Brand:</span>
									<span class="ml-1">{boat.brand}</span>
								</div>
							{/if}
							{#if boat.model}
								<div class="flex items-center">
									<span class="font-medium">Model:</span>
									<span class="ml-1">{boat.model}</span>
								</div>
							{/if}
							{#if boat.sailNumber}
								<div class="flex items-center">
									<span class="font-medium">Sail Number:</span>
									<span class="ml-1">{boat.sailNumber}</span>
								</div>
							{/if}
						</div>

						<div class="mt-4 flex items-center justify-between">
							<span
								class="inline-flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800"
							>
								Registered
							</span>

							<div class="flex space-x-2">
								<button class="text-sm font-medium text-blue-600 hover:text-blue-800">
									View
								</button>
								{#if $userStore}
									<button class="text-sm font-medium text-gray-600 hover:text-gray-800">
										Edit
									</button>
								{/if}
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
