<script lang="ts">
	import { boatsApi } from '../api/boats';
	import type { Country, BoatCreate, Boat } from '../api/types';

	interface Props {
		isOpen: boolean;
		countries: Country[];
		editingBoat?: Boat | null;
		onClose: () => void;
		onBoatCreated: () => void;
	}

	const { isOpen, countries, editingBoat = null, onClose, onBoatCreated }: Props = $props();

	// Form state
	let formData = $state<BoatCreate>({
		name: editingBoat?.name || '',
		brand: editingBoat?.brand || '',
		model: editingBoat?.model || '',
		sailNumber: editingBoat?.sailNumber || '',
		countryId: editingBoat?.countryId || ''
	});

	// Update form when editingBoat changes
	$effect(() => {
		if (editingBoat) {
			formData = {
				name: editingBoat.name,
				brand: editingBoat.brand || '',
				model: editingBoat.model || '',
				sailNumber: editingBoat.sailNumber || '',
				countryId: editingBoat.countryId
			};
		} else {
			formData = {
				name: '',
				brand: '',
				model: '',
				sailNumber: '',
				countryId: ''
			};
		}
	});

	let errors = $state<Record<string, string>>({});
	let isSubmitting = $state(false);

	// Validation rules based on backend
	const SAIL_NUMBER_REGEX = /^[A-Z]{3}\d{1,5}$/;

	function validateForm(): boolean {
		const newErrors: Record<string, string> = {};

		// Name validation
		if (!formData.name.trim()) {
			newErrors.name = 'Boat name is required';
		} else if (formData.name.trim().length < 2) {
			newErrors.name = 'Boat name must contain at least 2 characters';
		}

		// Brand validation (optional but if provided, min 1 char)
		if (formData.brand && formData.brand.trim().length < 1) {
			newErrors.brand = 'Brand must contain at least 1 character';
		}

		// Model validation (optional but if provided, min 1 char)
		if (formData.model && formData.model.trim().length < 1) {
			newErrors.model = 'Model must contain at least 1 character';
		}

		// Sail number validation (optional but if provided, must match format)
		if (formData.sailNumber && formData.sailNumber.trim()) {
			if (!SAIL_NUMBER_REGEX.test(formData.sailNumber.trim())) {
				newErrors.sailNumber =
					'Sail number must be in format: 3 uppercase letters followed by 1-5 digits (e.g., NOR5828)';
			}
		}

		// Country validation
		if (!formData.countryId) {
			newErrors.countryId = 'Country is required';
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	async function handleSubmit() {
		if (!validateForm()) {
			return;
		}

		isSubmitting = true;
		try {
			// Clean up form data - convert empty strings to undefined for optional fields
			const cleanedData: BoatCreate = {
				name: formData.name.trim(),
				brand: formData.brand?.trim() || undefined,
				model: formData.model?.trim() || undefined,
				sailNumber: formData.sailNumber?.trim() || undefined,
				countryId: formData.countryId
			};

			if (editingBoat) {
				await boatsApi.updateBoat(editingBoat.id, cleanedData);
			} else {
				await boatsApi.createBoat(cleanedData);
			}

			// Reset form
			formData = {
				name: '',
				brand: '',
				model: '',
				sailNumber: '',
				countryId: ''
			};
			errors = {};

			onBoatCreated();
		} catch (error) {
			console.error(`Failed to ${editingBoat ? 'update' : 'create'} boat:`, error);
			errors.submit =
				error instanceof Error
					? error.message
					: `Failed to ${editingBoat ? 'update' : 'create'} boat`;
		} finally {
			isSubmitting = false;
		}
	}

	function handleClose() {
		// Reset form when closing
		formData = {
			name: '',
			brand: '',
			model: '',
			sailNumber: '',
			countryId: ''
		};
		errors = {};
		onClose();
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			handleClose();
		}
	}
</script>

<!-- Drawer overlay -->
{#if isOpen}
	<div
		class="fixed inset-0 z-50 overflow-hidden"
		onclick={handleBackdropClick}
		role="dialog"
		aria-modal="true"
		aria-labelledby="drawer-title"
	>
		<!-- Drawer panel -->
		<div class="fixed right-0 top-0 h-full w-full max-w-md bg-white shadow-xl">
			<div class="flex h-full flex-col">
				<!-- Header -->
				<div class="border-b border-gray-200 px-6 py-4">
					<div class="flex items-center justify-between">
						<h2 id="drawer-title" class="text-lg font-medium text-gray-900">
							{editingBoat ? 'Edit Boat' : 'Add New Boat'}
						</h2>
						<button
							onclick={handleClose}
							class="rounded-md text-gray-400 hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
						>
							<span class="sr-only">Close</span>
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
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

				<!-- Form -->
				<div class="flex-1 overflow-y-auto px-6 py-6">
					<form onsubmit={handleSubmit} class="space-y-6">
						<!-- Boat Name -->
						<div>
							<label for="name" class="block text-sm font-medium text-gray-700">
								Boat Name <span class="text-red-500">*</span>
							</label>
							<input
								id="name"
								type="text"
								bind:value={formData.name}
								class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
								class:border-red-300={errors.name}
								class:ring-red-500={errors.name}
								class:focus:border-red-500={errors.name}
								class:focus:ring-red-500={errors.name}
								placeholder="Enter boat name"
								required
							/>
							{#if errors.name}
								<p class="mt-1 text-sm text-red-600">{errors.name}</p>
							{/if}
						</div>

						<!-- Brand -->
						<div>
							<label for="brand" class="block text-sm font-medium text-gray-700">Brand</label>
							<input
								id="brand"
								type="text"
								bind:value={formData.brand}
								class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
								class:border-red-300={errors.brand}
								class:ring-red-500={errors.brand}
								class:focus:border-red-500={errors.brand}
								class:focus:ring-red-500={errors.brand}
								placeholder="Enter brand (optional)"
							/>
							{#if errors.brand}
								<p class="mt-1 text-sm text-red-600">{errors.brand}</p>
							{/if}
						</div>

						<!-- Model -->
						<div>
							<label for="model" class="block text-sm font-medium text-gray-700">Model</label>
							<input
								id="model"
								type="text"
								bind:value={formData.model}
								class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
								class:border-red-300={errors.model}
								class:ring-red-500={errors.model}
								class:focus:border-red-500={errors.model}
								class:focus:ring-red-500={errors.model}
								placeholder="Enter model (optional)"
							/>
							{#if errors.model}
								<p class="mt-1 text-sm text-red-600">{errors.model}</p>
							{/if}
						</div>

						<!-- Sail Number -->
						<div>
							<label for="sailNumber" class="block text-sm font-medium text-gray-700"
								>Sail Number</label
							>
							<input
								id="sailNumber"
								type="text"
								bind:value={formData.sailNumber}
								class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
								class:border-red-300={errors.sailNumber}
								class:ring-red-500={errors.sailNumber}
								class:focus:border-red-500={errors.sailNumber}
								class:focus:ring-red-500={errors.sailNumber}
								placeholder="e.g., NOR5828"
								pattern="[A-Z]{3}\d{1,5}"
								title="3 uppercase letters followed by 1-5 digits"
							/>
							{#if errors.sailNumber}
								<p class="mt-1 text-sm text-red-600">{errors.sailNumber}</p>
							{:else}
								<p class="mt-1 text-sm text-gray-500">
									Format: 3 uppercase letters + 1-5 digits (optional)
								</p>
							{/if}
						</div>

						<!-- Country -->
						<div>
							<label for="countryId" class="block text-sm font-medium text-gray-700">
								Country <span class="text-red-500">*</span>
							</label>
							<select
								id="countryId"
								bind:value={formData.countryId}
								class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
								class:border-red-300={errors.countryId}
								class:ring-red-500={errors.countryId}
								class:focus:border-red-500={errors.countryId}
								class:focus:ring-red-500={errors.countryId}
								required
							>
								<option value="">Select a country</option>
								{#each countries as country}
									<option value={country.id}>{country.name} ({country.isoAlpha2})</option>
								{/each}
							</select>
							{#if errors.countryId}
								<p class="mt-1 text-sm text-red-600">{errors.countryId}</p>
							{/if}
						</div>

						<!-- Submit Error -->
						{#if errors.submit}
							<div class="rounded-md bg-red-50 p-4">
								<div class="flex">
									<svg
										class="h-5 w-5 text-red-400"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
										/>
									</svg>
									<div class="ml-3">
										<p class="text-sm text-red-800">{errors.submit}</p>
									</div>
								</div>
							</div>
						{/if}
					</form>
				</div>

				<!-- Footer -->
				<div class="border-t border-gray-200 px-6 py-4">
					<div class="flex justify-end space-x-3">
						<button
							type="button"
							onclick={handleClose}
							class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
						>
							Cancel
						</button>
						<button
							type="submit"
							onclick={handleSubmit}
							disabled={isSubmitting}
							class="rounded-md border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
						>
							{#if isSubmitting}
								<div class="flex items-center">
									<div
										class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent"
									></div>
									{editingBoat ? 'Updating...' : 'Creating...'}
								</div>
							{:else}
								{editingBoat ? 'Update Boat' : 'Create Boat'}
							{/if}
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
