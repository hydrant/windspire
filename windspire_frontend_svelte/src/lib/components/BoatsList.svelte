<script lang="ts">
	import type { Boat, PaginatedResult } from '../api/types';

	interface Props {
		boats: PaginatedResult<Boat>;
		onPageChange: (page: number) => void;
		onPageSizeChange: (size: number) => void;
		onEditBoat: (boat: Boat) => void;
		onDeleteBoat: (boat: Boat) => void;
	}

	const { boats, onPageChange, onPageSizeChange, onEditBoat, onDeleteBoat }: Props = $props();

	function handlePageChange(page: number) {
		onPageChange(page);
	}

	function handlePageSizeChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		onPageSizeChange(parseInt(target.value));
	}

	function getPageNumbers(): number[] {
		const totalPages = boats.totalPages;
		const currentPage = boats.page;
		const pages: number[] = [];

		// Show up to 5 page numbers around current page
		const start = Math.max(1, currentPage - 2);
		const end = Math.min(totalPages, currentPage + 2);

		for (let i = start; i <= end; i++) {
			pages.push(i);
		}

		return pages;
	}
</script>

{#if boats.data.length === 0}
	<div class="py-12 text-center">
		<svg
			class="mx-auto h-12 w-12 text-gray-400"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
			/>
		</svg>
		<h3 class="mt-2 text-sm font-medium text-gray-900">No boats</h3>
		<p class="mt-1 text-sm text-gray-500">Get started by creating a new boat.</p>
	</div>
{:else}
	<!-- Boats Grid -->
	<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
		{#each boats.data as boat (boat.id)}
			<div
				class="overflow-hidden rounded-lg bg-white shadow transition-shadow duration-200 hover:shadow-md"
			>
				<div class="p-6">
					<div class="flex items-center justify-between">
						<h3 class="truncate text-lg font-medium text-gray-900">{boat.name}</h3>
						{#if boat.sailNumber}
							<span
								class="inline-flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800"
							>
								{boat.sailNumber}
							</span>
						{/if}
					</div>

					<div class="mt-4 space-y-2">
						{#if boat.brand}
							<div class="flex items-center text-sm text-gray-600">
								<svg
									class="mr-2 h-4 w-4 text-gray-400"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
									/>
								</svg>
								<span class="font-medium">Brand:</span>
								<span class="ml-1">{boat.brand}</span>
							</div>
						{/if}

						{#if boat.model}
							<div class="flex items-center text-sm text-gray-600">
								<svg
									class="mr-2 h-4 w-4 text-gray-400"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
									/>
								</svg>
								<span class="font-medium">Model:</span>
								<span class="ml-1">{boat.model}</span>
							</div>
						{/if}

						<!-- Owners -->
						{#if boat.owners && boat.owners.length > 0}
							<div class="flex items-center text-sm text-gray-600">
								<svg
									class="mr-2 h-4 w-4 text-gray-400"
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
								<span class="font-medium">Owners:</span>
								<span class="ml-1">
									{#each boat.owners as owner, index (owner.id)}
										{owner.firstName} {owner.lastName}{index < boat.owners.length - 1 ? ', ' : ''}
									{/each}
								</span>
							</div>
						{:else}
							<div class="flex items-center text-sm text-gray-500">
								<svg
									class="mr-2 h-4 w-4 text-gray-400"
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
								<span class="font-medium">No owners assigned</span>
							</div>
						{/if}
					</div>

					<div class="mt-6 flex justify-end space-x-2">
						<button
							onclick={() => onEditBoat(boat)}
							class="text-sm font-medium text-blue-600 hover:text-blue-800"
						>
							Edit
						</button>
						<button
							onclick={() => onDeleteBoat(boat)}
							class="text-sm font-medium text-red-600 hover:text-red-800"
						>
							Delete
						</button>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<!-- Pagination -->
	<div class="mt-8 flex items-center justify-between">
		<div class="flex items-center space-x-2">
			<span class="text-sm text-gray-700">Show</span>
			<select
				onchange={handlePageSizeChange}
				value={boats.limit}
				class="rounded-md border border-gray-300 px-3 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
			>
				<option value="10">10</option>
				<option value="20">20</option>
				<option value="50">50</option>
				<option value="100">100</option>
			</select>
			<span class="text-sm text-gray-700">per page</span>
		</div>

		<div class="flex items-center space-x-2">
			<span class="text-sm text-gray-700">
				Showing {(boats.page - 1) * boats.limit + 1} to {Math.min(
					boats.page * boats.limit,
					boats.total
				)} of {boats.total} results
			</span>
		</div>

		<div class="flex items-center space-x-1">
			<!-- Previous button -->
			<button
				onclick={() => handlePageChange(boats.page - 1)}
				disabled={boats.page <= 1}
				class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
			>
				Previous
			</button>

			<!-- Page numbers -->
			{#each getPageNumbers() as pageNum (pageNum)}
				<button
					onclick={() => handlePageChange(pageNum)}
					class="rounded-md px-3 py-2 text-sm font-medium"
					class:bg-blue-600={pageNum === boats.page}
					class:text-white={pageNum === boats.page}
					class:text-gray-700={pageNum !== boats.page}
					class:bg-white={pageNum !== boats.page}
					class:border={pageNum !== boats.page}
					class:border-gray-300={pageNum !== boats.page}
					class:hover:bg-gray-50={pageNum !== boats.page}
				>
					{pageNum}
				</button>
			{/each}

			<!-- Next button -->
			<button
				onclick={() => handlePageChange(boats.page + 1)}
				disabled={boats.page >= boats.totalPages}
				class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
			>
				Next
			</button>
		</div>
	</div>
{/if}
