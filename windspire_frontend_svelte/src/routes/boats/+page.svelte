<script lang="ts">
	import { onMount } from 'svelte';
	import { boatsApi } from '../../lib/api/boats';
	import { countriesApi } from '../../lib/api/countries';
	import type { Boat, Country, PaginatedResult } from '../../lib/api/types';
	import BoatsList from '../../lib/components/BoatsList.svelte';
	import BoatDrawer from '../../lib/components/BoatDrawer.svelte';
	import ConfirmationDrawer from '../../lib/components/ConfirmationDrawer.svelte';
	import { userStore } from '../../lib/stores/user';

	// Get user from store
	let user = $state($userStore);

	// Subscribe to user store changes
	$effect(() => {
		const unsubscribe = userStore.subscribe((value) => {
			user = value;
		});
		return unsubscribe;
	});

	let boats = $state<PaginatedResult<Boat> | null>(null);
	let countries = $state<Country[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let currentPage = $state(1);
	let pageSize = $state(20);
	let isDrawerOpen = $state(false);
	let editingBoat = $state<Boat | null>(null);
	let isDeleteModalOpen = $state(false);
	let boatToDelete = $state<Boat | null>(null);

	async function loadBoats(page: number = currentPage, limit: number = pageSize) {
		try {
			loading = true;
			error = null;
			boats = await boatsApi.getBoats({ page, limit });
			currentPage = page;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load boats';
			console.error('Failed to load boats:', err);
		} finally {
			loading = false;
		}
	}

	async function loadCountries() {
		try {
			countries = await countriesApi.getCountries();
		} catch (err) {
			console.error('Failed to load countries:', err);
		}
	}

	function handlePageChange(page: number) {
		loadBoats(page, pageSize);
	}

	function handlePageSizeChange(size: number) {
		pageSize = size;
		loadBoats(1, size);
	}

	function openDrawer() {
		editingBoat = null;
		isDrawerOpen = true;
	}

	function closeDrawer() {
		editingBoat = null;
		isDrawerOpen = false;
	}

	async function handleBoatCreated() {
		closeDrawer();
		await loadBoats(currentPage, pageSize);
	}

	function handleEditBoat(boat: Boat) {
		editingBoat = boat;
		isDrawerOpen = true;
	}

	function handleDeleteBoat(boat: Boat) {
		boatToDelete = boat;
		isDeleteModalOpen = true;
	}

	async function confirmDelete() {
		if (!boatToDelete) return;

		try {
			await boatsApi.deleteBoat(boatToDelete.id);
			isDeleteModalOpen = false;
			boatToDelete = null;
			await loadBoats(currentPage, pageSize);
		} catch (err) {
			console.error('Failed to delete boat:', err);
			// You could show an error message here
		}
	}

	function cancelDelete() {
		isDeleteModalOpen = false;
		boatToDelete = null;
	}

	onMount(() => {
		loadBoats();
		loadCountries();
	});

	// Reactive effect to reload boats when user authentication changes
	$effect(() => {
		// When user changes (login/logout), reload boats
		if (user !== null) {
			// User is now logged in, reload boats
			loadBoats();
		}
	});
</script>

<svelte:head>
	<title>Boats - Windspire</title>
	<meta name="description" content="Manage your boats with Windspire" />
</svelte:head>

<div class="min-h-screen bg-gray-50">
	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold text-gray-900">Boats</h1>
					<p class="mt-2 text-gray-600">Manage your fleet of boats</p>
				</div>
				<button
					onclick={openDrawer}
					class="flex items-center space-x-2 rounded-lg bg-blue-600 px-4 py-2 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
				>
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 4v16m8-8H4"
						/>
					</svg>
					<span>Add Boat</span>
				</button>
			</div>
		</div>

		<!-- Content -->
		{#if loading}
			<div class="flex items-center justify-center py-12">
				<div class="h-8 w-8 animate-spin rounded-full border-b-2 border-blue-600"></div>
				<span class="ml-3 text-gray-600">Loading boats...</span>
			</div>
		{:else if error}
			<div class="rounded-lg border border-red-200 bg-red-50 p-4">
				<div class="flex">
					<svg class="h-5 w-5 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
					<div class="ml-3">
						<h3 class="text-sm font-medium text-red-800">Error loading boats</h3>
						<p class="mt-1 text-sm text-red-700">{error}</p>
						<button
							onclick={() => loadBoats()}
							class="mt-2 text-sm text-red-800 underline hover:text-red-900"
						>
							Try again
						</button>
					</div>
				</div>
			</div>
		{:else if boats}
			<BoatsList
				{boats}
				onPageChange={handlePageChange}
				onPageSizeChange={handlePageSizeChange}
				onEditBoat={handleEditBoat}
				onDeleteBoat={handleDeleteBoat}
			/>
		{/if}
	</div>
</div>

<!-- Boat Creation/Edit Drawer -->
<BoatDrawer
	isOpen={isDrawerOpen}
	{countries}
	{editingBoat}
	onClose={closeDrawer}
	onBoatCreated={handleBoatCreated}
/>

<!-- Delete Confirmation Drawer -->
<ConfirmationDrawer
	isOpen={isDeleteModalOpen}
	title="Delete Boat"
	message={boatToDelete ? `Are you sure you want to delete "${boatToDelete.name}"?` : ''}
	confirmText="Delete"
	cancelText="Cancel"
	onConfirm={confirmDelete}
	onCancel={cancelDelete}
/>
