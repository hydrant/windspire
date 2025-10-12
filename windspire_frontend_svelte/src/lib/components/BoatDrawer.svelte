<script lang="ts">
	import { boatsApi } from '../api/boats';
	import type { Country, BoatCreate, Boat, Owner } from '../api/types';
	import { userStore } from '../stores/user';
	import UserSearchSelect from './UserSearchSelect.svelte';

	// Owner management state
	let ownerLoading = $state(false);
	let ownerError = $state('');
	let ownerSuccess = $state('');

	// Get user from store
	let user = $state($userStore);

	// Subscribe to user store changes
	$effect(() => {
		const unsubscribe = userStore.subscribe((value) => {
			user = value;
		});
		return unsubscribe;
	});

	async function handleAddOwner(selectedUser: Owner) {
		if (!editingBoat) return;

		// Check if user is already an owner
		if (editingBoat.owners?.some((owner) => owner.id === selectedUser.id)) {
			ownerError = `${selectedUser.firstName} ${selectedUser.lastName} is already an owner of this boat.`;
			setTimeout(() => (ownerError = ''), 3000);
			return;
		}

		ownerLoading = true;
		ownerError = '';
		ownerSuccess = '';

		try {
			await boatsApi.addOwnerToBoat(editingBoat.id, selectedUser.id);

			// Update the local boat state immediately
			if (editingBoat.owners) {
				editingBoat.owners = [...editingBoat.owners, selectedUser];
			} else {
				editingBoat.owners = [selectedUser];
			}

			ownerSuccess = `${selectedUser.firstName} ${selectedUser.lastName} has been added as an owner.`;
			setTimeout(() => (ownerSuccess = ''), 3000);

			// Note: Not calling onBoatCreated() to avoid closing the drawer
			// The main boat list will be refreshed when the drawer is closed
		} catch (e) {
			ownerError = e instanceof Error ? e.message : 'Failed to add owner';
			console.error(`Adding owner to ${editingBoat.id} failed:`, ownerError);
		} finally {
			ownerLoading = false;
		}
	}

	async function handleRemoveOwner(owner: Owner) {
		if (!editingBoat) return;

		// Prevent removing the last owner
		if (editingBoat.owners && editingBoat.owners.length <= 1) {
			ownerError = 'Cannot remove the last owner. A boat must have at least one owner.';
			setTimeout(() => (ownerError = ''), 3000);
			return;
		}

		ownerLoading = true;
		ownerError = '';
		ownerSuccess = '';

		try {
			await boatsApi.removeOwnerFromBoat(editingBoat.id, owner.id);

			// Update the local boat state immediately
			if (editingBoat.owners) {
				editingBoat.owners = editingBoat.owners.filter((o) => o.id !== owner.id);
			}

			ownerSuccess = `${owner.firstName} ${owner.lastName} has been removed as an owner.`;
			setTimeout(() => (ownerSuccess = ''), 3000);

			// Note: Not calling onBoatCreated() to avoid closing the drawer
			// The main boat list will be refreshed when the drawer is closed
		} catch (e) {
			ownerError = e instanceof Error ? e.message : 'Failed to remove owner';
		} finally {
			ownerLoading = false;
		}
	}

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

		// Check if user is logged in when creating a new boat
		if (!editingBoat && !user) {
			errors.submit = 'You must be logged in to create a boat';
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
				// Create the boat first
				const newBoat = await boatsApi.createBoat(cleanedData);

				// Then add the current user as an owner
				if (user) {
					try {
						await boatsApi.addOwnerToBoat(newBoat.id, user.id);
					} catch (ownerError) {
						console.error('Failed to add user as owner:', ownerError);
						// Note: We don't fail the entire operation if adding owner fails
						// The boat is created but without an owner
					}
				}
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

		// Refresh the main boat list if we were editing a boat (to sync any owner changes)
		if (editingBoat && typeof onBoatCreated === 'function') {
			onBoatCreated();
		}

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
		<div class="fixed top-0 right-0 h-full w-full max-w-md min-w-[30vw] bg-white shadow-xl">
			<div class="flex h-full flex-col">
				<!-- Header -->
				<div class="border-b border-gray-200 px-6 py-4">
					<div class="flex items-center justify-between">
						<h2 id="drawer-title" class="text-lg font-medium text-gray-900">
							{editingBoat ? 'Edit Boat' : 'Add New Boat'}
						</h2>
						<button
							onclick={handleClose}
							class="rounded-md text-gray-400 hover:text-gray-600 focus:ring-2 focus:ring-blue-500 focus:outline-none"
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
				<div class="flex-1 overflow-y-auto px-6 pt-6">
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
								pattern="[A-Z]{3}\d{5}"
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

				<!-- Owner Management (only when editing) -->
				{#if editingBoat}
					<div class="px-6 pt-2 pb-6">
						<h3 class="mb-3 text-lg font-medium text-gray-900">Boat Owners</h3>

						<!-- Add New Owner -->
						<div class="mb-4">
							<label class="mb-2 block text-sm font-medium text-gray-700"> Add New Owner </label>
							<UserSearchSelect
								onSelect={handleAddOwner}
								excludeUserIds={editingBoat.owners?.map((o) => o.id) || []}
								placeholder="Search for users to add as owners..."
								disabled={ownerLoading}
							/>
						</div>

						<!-- Current Owners -->
						<div class="mb-4">
							{#if editingBoat.owners && editingBoat.owners.length > 0}
								<div class="max-h-48 overflow-y-auto rounded-lg border border-gray-200 p-1">
									<div class="space-y-1">
										{#each editingBoat.owners as owner, index}
											<div
												class="flex items-center justify-between rounded bg-gray-50 p-2 {index > 0
													? ''
													: ''}"
											>
												<div class="flex items-center space-x-3">
													<!-- Avatar -->
													{#if owner.avatarUrl}
														<img
															class="h-8 w-8 rounded-full"
															src={owner.avatarUrl}
															alt="{owner.firstName} {owner.lastName}"
														/>
													{:else}
														<div
															class="flex h-8 w-8 items-center justify-center rounded-full bg-blue-100"
														>
															<span class="text-xs font-medium text-blue-700">
																{owner.firstName.charAt(0)}{owner.lastName.charAt(0)}
															</span>
														</div>
													{/if}

													<!-- Owner Info -->
													<div>
														<p class="text-sm font-medium text-gray-900">
															{owner.firstName}
															{owner.lastName}
														</p>
														<p class="text-sm text-gray-500">{owner.email}</p>
														{#if owner.isoName}
															<p class="text-xs text-gray-400">{owner.isoName}</p>
														{/if}
													</div>
												</div>

												<!-- Remove Button -->
												<button
													type="button"
													onclick={() => handleRemoveOwner(owner)}
													disabled={ownerLoading || editingBoat.owners.length <= 1}
													class="inline-flex items-center rounded-md border border-transparent bg-red-100 px-3 py-1.5 text-xs font-medium text-red-700 hover:bg-red-200 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
													title={editingBoat.owners.length <= 1
														? 'Cannot remove the last owner'
														: 'Remove owner'}
												>
													{#if ownerLoading}
														<div
															class="mr-1 h-3 w-3 animate-spin rounded-full border border-red-700 border-t-transparent"
														></div>
													{:else}
														<svg
															class="mr-1 h-3 w-3"
															fill="none"
															viewBox="0 0 24 24"
															stroke="currentColor"
														>
															<path
																stroke-linecap="round"
																stroke-linejoin="round"
																stroke-width="2"
																d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
															/>
														</svg>
													{/if}
													Remove
												</button>
											</div>
										{/each}
									</div>
								</div>
							{:else}
								<div class="py-4 text-center">
									<svg
										class="mx-auto h-8 w-8 text-gray-400"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z"
										/>
									</svg>
									<p class="mt-1 text-sm text-gray-500">No owners assigned yet</p>
								</div>
							{/if}
						</div>

						<!-- Messages -->
						{#if ownerError}
							<div class="mt-3 rounded-md bg-red-50 p-3">
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
										<p class="text-sm text-red-800">{ownerError}</p>
									</div>
								</div>
							</div>
						{/if}

						{#if ownerSuccess}
							<div class="mt-3 rounded-md bg-green-50 p-3">
								<div class="flex">
									<svg
										class="h-5 w-5 text-green-400"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
										/>
									</svg>
									<div class="ml-3">
										<p class="text-sm text-green-800">{ownerSuccess}</p>
									</div>
								</div>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Footer -->
				<div class="border-t border-gray-200 px-6 py-4">
					<div class="flex justify-end space-x-3">
						<button
							type="button"
							onclick={handleClose}
							class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
						>
							Cancel
						</button>
						<button
							type="submit"
							onclick={handleSubmit}
							disabled={isSubmitting}
							class="rounded-md border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
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
