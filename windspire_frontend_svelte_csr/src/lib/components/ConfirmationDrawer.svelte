<script lang="ts">
	interface Props {
		isOpen: boolean;
		title: string;
		message: string;
		confirmText?: string;
		cancelText?: string;
		confirmButtonClass?: string;
		onConfirm: () => void;
		onCancel: () => void;
	}

	const {
		isOpen,
		title,
		message,
		confirmText = 'Confirm',
		cancelText = 'Cancel',
		confirmButtonClass = 'bg-red-600 hover:bg-red-700 focus:ring-red-500',
		onConfirm,
		onCancel
	}: Props = $props();

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			onCancel();
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
		<div class="fixed right-0 top-0 h-full w-full min-w-[30vw] max-w-md bg-white shadow-xl">
			<div class="flex h-full flex-col">
				<!-- Header -->
				<div class="border-b border-gray-200 px-6 py-4">
					<div class="flex items-center justify-between">
						<h2 id="drawer-title" class="text-lg font-medium text-gray-900">
							{title}
						</h2>
						<button
							onclick={onCancel}
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

				<!-- Content -->
				<div class="flex-1 overflow-y-auto px-6 pt-6">
					<div class="flex items-start space-x-4">
						<!-- Warning Icon -->
						<div class="flex-shrink-0">
							<div class="flex h-12 w-12 items-center justify-center rounded-full bg-red-100">
								<svg
									class="h-6 w-6 text-red-600"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
									/>
								</svg>
							</div>
						</div>

						<!-- Message -->
						<div class="flex-1">
							<p class="text-sm leading-relaxed text-gray-700">
								{message}
							</p>
						</div>
					</div>

					<!-- Warning -->
					<div class="mt-8 rounded-md bg-red-50 p-4">
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
								<p class="text-sm text-red-800">This action cannot be undone.</p>
							</div>
						</div>
					</div>
				</div>

				<!-- Footer -->
				<div class="border-t border-gray-200 px-6 py-4">
					<div class="flex justify-end space-x-3">
						<button
							type="button"
							onclick={onCancel}
							class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
						>
							{cancelText}
						</button>
						<button
							type="button"
							onclick={onConfirm}
							class="rounded-md border border-transparent px-4 py-2 text-sm font-medium text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 {confirmButtonClass}"
						>
							{confirmText}
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
