<script lang="ts">
	import { usersApi } from '../api/users';
	import type { Owner } from '../api/types';

	interface Props {
		onSelect: (user: Owner) => void;
		excludeUserIds?: string[];
		placeholder?: string;
		disabled?: boolean;
	}

	const {
		onSelect,
		excludeUserIds = [],
		placeholder = 'Search users...',
		disabled = false
	}: Props = $props();

	let searchQuery = $state('');
	let searchResults = $state<Owner[]>([]);
	let isSearching = $state(false);
	let showDropdown = $state(false);
	let searchTimeout: ReturnType<typeof setTimeout> | null = null;

	async function performSearch(query: string) {
		if (!query.trim()) {
			searchResults = [];
			showDropdown = false;
			return;
		}

		isSearching = true;
		try {
			const results = await usersApi.searchUsers(query);
			// Filter out excluded users
			searchResults = results.filter((user) => !excludeUserIds.includes(user.id));
			showDropdown = true;
		} catch (error) {
			console.error('Failed to search users:', error);
			searchResults = [];
		} finally {
			isSearching = false;
		}
	}

	function handleSearchInput() {
		// Clear previous timeout
		if (searchTimeout) {
			clearTimeout(searchTimeout);
		}

		// Debounce search
		searchTimeout = setTimeout(() => {
			performSearch(searchQuery);
		}, 300);
	}

	function handleSelectUser(user: Owner) {
		onSelect(user);
		searchQuery = '';
		searchResults = [];
		showDropdown = false;
	}

	function handleInputFocus() {
		if (searchQuery.trim() && searchResults.length > 0) {
			showDropdown = true;
		}
	}

	function handleInputBlur() {
		// Delay hiding dropdown to allow clicks on results
		setTimeout(() => {
			showDropdown = false;
		}, 200);
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			showDropdown = false;
			searchQuery = '';
		}
	}
</script>

<div class="relative">
	<div class="relative">
		<input
			type="text"
			bind:value={searchQuery}
			oninput={handleSearchInput}
			onfocus={handleInputFocus}
			onblur={handleInputBlur}
			onkeydown={handleKeydown}
			{placeholder}
			{disabled}
			class="w-full rounded-md border border-gray-300 bg-white py-2 pr-10 pl-3 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none disabled:bg-gray-50 disabled:text-gray-500"
		/>

		<!-- Search icon -->
		<div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
			{#if isSearching}
				<div
					class="h-4 w-4 animate-spin rounded-full border-2 border-gray-300 border-t-blue-600"
				></div>
			{:else}
				<svg class="h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
			{/if}
		</div>
	</div>

	<!-- Dropdown -->
	{#if showDropdown && searchResults.length > 0}
		<div
			class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md border border-gray-200 bg-white shadow-lg"
		>
			{#each searchResults as user (user.id)}
				<button
					type="button"
					onclick={() => handleSelectUser(user)}
					class="w-full border-b border-gray-100 px-4 py-3 text-left last:border-b-0 hover:bg-gray-50 focus:bg-gray-50 focus:outline-none"
				>
					<div class="flex items-center space-x-3">
						<!-- Avatar placeholder -->
						<div class="flex-shrink-0">
							{#if user.avatarUrl}
								<img
									class="h-8 w-8 rounded-full"
									src={user.avatarUrl}
									alt="{user.firstName} {user.lastName}"
								/>
							{:else}
								<div class="flex h-8 w-8 items-center justify-center rounded-full bg-gray-300">
									<span class="text-sm font-medium text-gray-700">
										{user.firstName.charAt(0)}{user.lastName.charAt(0)}
									</span>
								</div>
							{/if}
						</div>

						<!-- User info -->
						<div class="min-w-0 flex-1">
							<p class="truncate text-sm font-medium text-gray-900">
								{user.firstName}
								{user.lastName}
							</p>
							<p class="truncate text-sm text-gray-500">
								{user.email}
							</p>
							{#if user.isoName}
								<p class="text-xs text-gray-400">
									{user.isoName}
								</p>
							{/if}
						</div>
					</div>
				</button>
			{/each}
		</div>
	{:else if showDropdown && searchQuery.trim() && !isSearching}
		<div
			class="absolute z-10 mt-1 w-full rounded-md border border-gray-200 bg-white px-4 py-3 shadow-lg"
		>
			<p class="text-sm text-gray-500">No users found matching "{searchQuery}"</p>
		</div>
	{/if}
</div>
