<script lang="ts">
	import { onMount } from 'svelte';
	import { boatsApi, countriesApi } from '../api';
	import type { BoatCreate, Country } from '../api';

	type Props = {
		isOpen: boolean;
		onClose: () => void;
		onBoatCreated: () => void;
	};

	const { isOpen, onClose, onBoatCreated }: Props = $props();

	let formData = $state<BoatCreate>({
		name: '',
		brand: '',
		model: '',
		sailNumber: '',
		countryId: ''
	});

	let countries = $state<Country[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	// Load countries on mount
	onMount(async () => {
		try {
			console.log('Loading countries...');
			const token = localStorage.getItem('windspire_token');
			console.log('Auth token available:', !!token);
			const countriesResponse = await countriesApi.getCountries();
			console.log('Countries response:', countriesResponse);

			// Assign to state and use $state.snapshot to unwrap for logging
			countries = countriesResponse;
			console.log('Countries loaded:', countries.length, $state.snapshot(countries));
			console.log('First country object:', $state.snapshot(countries[0]));
		} catch (err) {
			console.error('Failed to load countries:', err);
			error = 'Failed to load countries. Please make sure you are logged in.';
		}
	});

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();

		if (!formData.name.trim() || !formData.countryId) {
			error = 'Name and country are required';
			return;
		}

		loading = true;
		error = null;

		try {
			// Clean up empty strings
			const cleanedData: BoatCreate = {
				name: formData.name.trim(),
				brand: formData.brand?.trim() || undefined,
				model: formData.model?.trim() || undefined,
				sailNumber: formData.sailNumber?.trim() || undefined,
				countryId: formData.countryId
			};

			console.log('Creating boat with data:', cleanedData);
			console.log('Auth token:', localStorage.getItem('windspire_token') ? 'present' : 'missing');

			await boatsApi.createBoat(cleanedData);

			// Reset form
			formData = {
				name: '',
				brand: '',
				model: '',
				sailNumber: '',
				countryId: ''
			};

			onBoatCreated();
			onClose();
		} catch (err) {
			console.error('Boat creation error:', err);
			if (err instanceof Error) {
				if (err.message.includes('403')) {
					error = 'You do not have permission to create boats. Please check your authentication.';
				} else if (err.message.includes('401')) {
					error = 'Authentication required. Please log in again.';
				} else {
					error = err.message;
				}
			} else {
				error = 'Failed to create boat';
			}
		} finally {
			loading = false;
		}
	}

	function handleCancel() {
		// Reset form and close
		formData = {
			name: '',
			brand: '',
			model: '',
			sailNumber: '',
			countryId: ''
		};
		error = null;
		onClose();
	}

	// Close modal on Escape key
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			handleCancel();
		}
	}
</script>

<svelte:window on:keydown={handleKeydown} />

{#if isOpen}
	<!-- Drawer backdrop -->
	<div
		class="fixed inset-0 z-50 overflow-hidden"
		role="dialog"
		aria-labelledby="drawer-title"
		aria-modal="true"
	>
		<!-- Background overlay -->
		<div class="absolute inset-0 overflow-hidden">
			<div
				class="absolute inset-0 bg-black bg-opacity-50 transition-opacity"
				onclick={handleCancel}
			></div>

			<!-- Drawer panel -->
			<div class="fixed inset-y-0 right-0 flex max-w-full pl-10">
				<div
					class="w-screen max-w-md transform bg-white shadow-xl transition-transform"
					onclick={(e) => e.stopPropagation()}
				>
					<!-- Drawer header -->
					<div class="flex h-full flex-col">
						<div class="bg-blue-600 px-4 py-6 sm:px-6">
							<div class="flex items-center justify-between">
								<h2 id="drawer-title" class="text-lg font-medium text-white">Add New Boat</h2>
								<div class="ml-3 flex h-7 items-center">
									<button
										onclick={handleCancel}
										class="rounded-md bg-blue-600 text-blue-200 hover:text-white focus:outline-none focus:ring-2 focus:ring-white"
										aria-label="Close drawer"
									>
										<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M6 18L18 6M6 6l12 12"
											/>
										</svg>
									</button>
								</div>
							</div>
						</div>

						<!-- Drawer content -->
						<div class="relative flex-1 overflow-y-auto px-4 py-6 sm:px-6">
							<form onsubmit={handleSubmit} class="space-y-6">
								<!-- Boat Name (Required) -->
								<div>
									<label for="name" class="block text-sm font-medium text-gray-700">
										Boat Name <span class="text-red-500">*</span>
									</label>
									<input
										id="name"
										type="text"
										bind:value={formData.name}
										required
										class="mt-1 block w-full rounded-md border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500"
										placeholder="Enter boat name"
									/>
								</div>

								<!-- Brand -->
								<div>
									<label for="brand" class="block text-sm font-medium text-gray-700">Brand</label>
									<input
										id="brand"
										type="text"
										bind:value={formData.brand}
										class="mt-1 block w-full rounded-md border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500"
										placeholder="e.g., Beneteau, Jeanneau"
									/>
								</div>

								<!-- Model -->
								<div>
									<label for="model" class="block text-sm font-medium text-gray-700">Model</label>
									<input
										id="model"
										type="text"
										bind:value={formData.model}
										class="mt-1 block w-full rounded-md border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500"
										placeholder="e.g., Oceanis 40, Sun Odyssey 349"
									/>
								</div>

								<!-- Sail Number -->
								<div>
									<label for="sailNumber" class="block text-sm font-medium text-gray-700">
										Sail Number
									</label>
									<input
										id="sailNumber"
										type="text"
										bind:value={formData.sailNumber}
										class="mt-1 block w-full rounded-md border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500"
										placeholder="e.g., NOR 123"
									/>
								</div>

								<!-- Country (Required) -->
								<div>
									<label for="country" class="block text-sm font-medium text-gray-700">
										Country <span class="text-red-500">*</span>
									</label>
									<select
										id="country"
										bind:value={formData.countryId}
										required
										class="mt-1 block w-full rounded-md border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500"
									>
										<option value=""
											>{countries.length > 0 ? 'Select a country' : 'Loading countries...'}</option
										>
										{#each countries as country}
											<option value={country.id}
												>{country.name ||
													country.isoAlpha3 ||
													country.isoAlpha2 ||
													country.id}</option
											>
										{/each}
									</select>
								</div>

								<!-- Error message -->
								{#if error}
									<div class="rounded-md bg-red-50 p-3">
										<div class="flex">
											<svg
												class="h-5 w-5 text-red-400"
												fill="currentColor"
												viewBox="0 0 20 20"
												aria-hidden="true"
											>
												<path
													fill-rule="evenodd"
													d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
													clip-rule="evenodd"
												/>
											</svg>
											<div class="ml-3">
												<p class="text-sm text-red-800">{error}</p>
											</div>
										</div>
									</div>
								{/if}

								<!-- Action buttons -->
								<div class="flex space-x-3 pt-6">
									<button
										type="button"
										onclick={handleCancel}
										class="flex-1 rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
									>
										Cancel
									</button>
									<button
										type="submit"
										disabled={loading}
										class="flex-1 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:bg-blue-300"
									>
										{loading ? 'Creating...' : 'Create Boat'}
									</button>
								</div>
							</form>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	input,
	select {
		border: 1px solid #d1d5db;
	}

	input:focus,
	select:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}
</style>
