<script lang="ts">
	import { firebaseAuthService } from '../firebase';
	import { userStore } from '../stores/user';
	import { config } from '../config';
	import { route } from '@mateothegreat/svelte5-router';

	const { closeLoginModal }: { closeLoginModal: () => void } = $props();

	let email = $state('');
	let password = $state('');
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	async function signInWithGoogle() {
		isLoading = true;
		error = null;

		try {
			const result = await firebaseAuthService.signInWithGoogle();
			if (result && result.user) {
				// Get Firebase ID token
				const idToken = await result.user.getIdToken();

				// Send to our backend for verification and JWT generation
				const response = await fetch(`${config.API_BASE_URL}/auth/firebase`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ id_token: idToken })
				});

				if (response.ok) {
					const data = await response.json();
					if (data.success && data.data) {
						// Store our backend JWT token
						localStorage.setItem('windspire_token', data.data.token);
						userStore.set(data.data.user);

						// Dispatch auth change event
						window.dispatchEvent(new CustomEvent('auth-changed'));

						closeLoginModal();
					} else {
						throw new Error(data.message || 'Authentication failed');
					}
				} else {
					throw new Error('Backend authentication failed');
				}
			}
		} catch (err: unknown) {
			console.error('Google sign-in error:', err);
			error = err instanceof Error ? err.message : 'Failed to sign in with Google';
		} finally {
			isLoading = false;
		}
	}

	async function signInWithEmail(event: SubmitEvent) {
		event.preventDefault();
		isLoading = true;
		error = null;

		try {
			const result = await firebaseAuthService.signInWithEmailAndPassword(email, password);
			if (result && result.user) {
				// Get Firebase ID token
				const idToken = await result.user.getIdToken();

				// Send to our backend for verification and JWT generation
				const response = await fetch(`${config.API_BASE_URL}/auth/firebase`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ id_token: idToken })
				});

				if (response.ok) {
					const data = await response.json();
					if (data.success && data.data) {
						// Store our backend JWT token
						localStorage.setItem('windspire_token', data.data.token);
						userStore.set(data.data.user);

						// Dispatch auth change event
						window.dispatchEvent(new CustomEvent('auth-changed'));

						closeLoginModal();
					} else {
						throw new Error(data.message || 'Authentication failed');
					}
				} else {
					throw new Error('Backend authentication failed');
				}
			}
		} catch (err: unknown) {
			console.error('Email sign-in error:', err);
			error = err instanceof Error ? err.message : 'Failed to sign in';
		} finally {
			isLoading = false;
		}
	}
</script>

<!-- Modal Backdrop -->
<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
	<!-- Modal Content -->
	<div class="mx-4 w-full max-w-md rounded-lg bg-white shadow-xl">
		<!-- Modal Header -->
		<div class="flex items-center justify-between border-b border-gray-200 p-6">
			<h2 class="text-xl font-semibold text-gray-900">Sign In</h2>
			<button
				onclick={closeLoginModal}
				class="text-gray-400 transition-colors duration-200 hover:text-gray-600"
				aria-label="Close modal"
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

		<!-- Modal Body -->
		<div class="p-6">
			{#if error}
				<div class="mb-4 rounded-lg border border-red-400 bg-red-100 p-3 text-red-700">
					{error}
				</div>
			{/if}

			<div class="space-y-4">
				<button
					onclick={signInWithGoogle}
					disabled={isLoading}
					class="w-full rounded-lg bg-blue-600 px-4 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-blue-400"
				>
					{#if isLoading}
						<svg
							class="-ml-1 mr-3 inline h-5 w-5 animate-spin text-white"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
						>
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
						Signing in...
					{:else}
						Sign in with Google
					{/if}
				</button>

				<div class="relative">
					<div class="absolute inset-0 flex items-center">
						<div class="w-full border-t border-gray-300"></div>
					</div>
					<div class="relative flex justify-center text-sm">
						<span class="bg-white px-2 text-gray-500">Or sign in with email</span>
					</div>
				</div>

				<form onsubmit={signInWithEmail} class="space-y-4">
					<div>
						<label for="modal-email" class="mb-1 block text-sm font-medium text-gray-700"
							>Email</label
						>
						<input
							type="email"
							id="modal-email"
							bind:value={email}
							disabled={isLoading}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
							placeholder="your.email@example.com"
							required
						/>
					</div>

					<div>
						<label for="modal-password" class="mb-1 block text-sm font-medium text-gray-700"
							>Password</label
						>
						<input
							type="password"
							id="modal-password"
							bind:value={password}
							disabled={isLoading}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
							placeholder="••••••••"
							required
						/>
					</div>

					<button
						type="submit"
						disabled={isLoading || !email || !password}
						class="w-full rounded-lg bg-blue-600 px-4 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-blue-400"
					>
						{#if isLoading}
							<svg
								class="-ml-1 mr-3 inline h-5 w-5 animate-spin text-white"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
							>
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
							Signing in...
						{:else}
							Sign In
						{/if}
					</button>
				</form>

				<div class="text-center">
					<a href="/auth" use:route class="text-sm text-blue-600 hover:text-blue-800">
						Don't have an account? Sign up
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
