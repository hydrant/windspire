<script lang="ts">
	import { firebaseAuth } from '$lib/firebase/auth';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onBackToLogin: () => void;
	}

	const { isOpen, onClose, onBackToLogin }: Props = $props();

	let email = $state('');
	let isLoading = $state(false);
	let error = $state('');
	let showSuccess = $state(false);

	function resetForm() {
		email = '';
		error = '';
		isLoading = false;
		showSuccess = false;
	}

	function handleClose() {
		resetForm();
		onClose();
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		error = '';

		if (!email) {
			error = 'Please enter your email address';
			return;
		}

		isLoading = true;

		try {
			console.log('Sending password reset email...');
			await firebaseAuth.sendPasswordReset(email);
			console.log('Password reset email sent successfully');
			showSuccess = true;
		} catch (err: unknown) {
			console.error('Password reset error:', err);
			error =
				err instanceof Error
					? err.message
					: 'Failed to send password reset email. Please try again.';
		} finally {
			isLoading = false;
		}
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			handleClose();
		}
	}
</script>

{#if isOpen}
	<!-- Modal backdrop -->
	<div
		class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-gray-600"
		onclick={handleBackdropClick}
	>
		<!-- Modal container -->
		<div class="relative w-full max-w-md rounded-lg bg-white p-6 shadow-xl">
			{#if showSuccess}
				<!-- Success message -->
				<div class="text-center">
					<div
						class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-green-100"
					>
						<svg
							class="h-6 w-6 text-green-600"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M5 13l4 4L19 7"
							/>
						</svg>
					</div>
					<h3 class="text-lg font-medium text-gray-900">Email Sent!</h3>
					<p class="mt-2 text-sm text-gray-500">
						We've sent a password reset link to {email}. Please check your email and follow the
						instructions to reset your password.
					</p>
					<div class="mt-4 space-y-2">
						<button
							onclick={onBackToLogin}
							class="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors duration-200 hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
						>
							Back to Sign In
						</button>
						<button
							onclick={handleClose}
							class="w-full rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 transition-colors duration-200 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
						>
							Close
						</button>
					</div>
				</div>
			{:else}
				<!-- Password reset form -->
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-xl font-medium text-gray-900">Reset Password</h3>
					<button
						onclick={handleClose}
						class="text-gray-400 hover:text-gray-600 focus:outline-none"
					>
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

				<p class="mb-4 text-sm text-gray-600">
					Enter your email address and we'll send you a link to reset your password.
				</p>

				{#if error}
					<div class="mb-4 rounded-md bg-red-50 p-3">
						<p class="text-sm text-red-700">{error}</p>
					</div>
				{/if}

				<form onsubmit={handleSubmit} class="space-y-4">
					<div>
						<label for="email" class="block text-sm font-medium text-gray-700">Email</label>
						<input
							type="email"
							id="email"
							bind:value={email}
							class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
							placeholder="Enter your email"
							required
						/>
					</div>

					<button
						type="submit"
						disabled={isLoading}
						class="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors duration-200 hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:cursor-not-allowed disabled:bg-gray-400"
					>
						{#if isLoading}
							<div class="flex items-center justify-center">
								<svg class="mr-2 h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
									<circle
										class="opacity-25"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
								Sending Email...
							</div>
						{:else}
							Send Reset Email
						{/if}
					</button>
				</form>

				<div class="mt-4 text-center">
					<button
						onclick={onBackToLogin}
						class="text-sm text-blue-600 hover:text-blue-700 focus:outline-none"
					>
						Back to Sign In
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}
