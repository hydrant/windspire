<script lang="ts">
	import { firebaseAuth } from '$lib/firebase/auth';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSuccess: () => void;
		onSwitchToRegister: () => void;
		onForgotPassword: () => void;
		onGoogleSignIn?: () => void;
	}

	const {
		isOpen,
		onClose,
		onSuccess,
		onSwitchToRegister,
		onForgotPassword,
		onGoogleSignIn
	}: Props = $props();

	let email = $state('');
	let password = $state('');
	let isLoading = $state(false);
	let error = $state('');

	function resetForm() {
		email = '';
		password = '';
		error = '';
		isLoading = false;
	}

	function handleClose() {
		resetForm();
		onClose();
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		error = '';

		// Validation
		if (!email || !password) {
			error = 'Please fill in all fields';
			return;
		}

		isLoading = true;

		try {
			console.log('Starting Firebase email/password sign-in...');

			// Sign in with email/password
			const result = await firebaseAuth.signInWithEmail(email, password);
			console.log('Firebase sign-in successful, sending to backend...');

			// Get ID token and send to backend
			const idToken = await result.user.getIdToken();

			// Include display name if available (for users with updated profiles)
			const requestBody: { id_token: string; display_name?: string } = { id_token: idToken };
			if (result.user.displayName) {
				requestBody.display_name = result.user.displayName;
				console.log('Sending display_name from Firebase profile:', result.user.displayName);
			}

			const response = await fetch('http://localhost:8080/auth/firebase', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(requestBody)
			});

			if (response.ok) {
				const data = await response.json();
				if (data.success && data.data) {
					// Store the JWT token from backend
					localStorage.setItem('windspire_token', data.data.token);

					// Trigger auth change event
					window.dispatchEvent(new CustomEvent('auth-changed'));

					console.log('Login successful');
					handleClose();
					onSuccess();
				} else {
					throw new Error(data.message || 'Login failed');
				}
			} else {
				const errorText = await response.text();
				throw new Error(`Backend authentication failed: ${response.status} ${errorText}`);
			}
		} catch (err: unknown) {
			console.error('Login error:', err);
			error = err instanceof Error ? err.message : 'Login failed. Please try again.';
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
			<!-- Login form -->
			<div class="mb-4 flex items-center justify-between">
				<h3 class="text-xl font-medium text-gray-900">Sign In</h3>
				<button onclick={handleClose} class="text-gray-400 hover:text-gray-600 focus:outline-none">
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

				<div>
					<label for="password" class="block text-sm font-medium text-gray-700">Password</label>
					<input
						type="password"
						id="password"
						bind:value={password}
						class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
						placeholder="Enter your password"
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
							Signing In...
						</div>
					{:else}
						Sign In
					{/if}
				</button>
			</form>

			<!-- Divider -->
			<div class="mt-6 mb-6">
				<div class="relative">
					<div class="absolute inset-0 flex items-center">
						<div class="w-full border-t border-gray-300"></div>
					</div>
					<div class="relative flex justify-center text-sm">
						<span class="bg-white px-2 text-gray-500">Or continue with</span>
					</div>
				</div>
			</div>

			<!-- Google Sign-In Button -->
			{#if onGoogleSignIn}
				<button
					type="button"
					onclick={onGoogleSignIn}
					class="w-full rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm transition-colors duration-200 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
				>
					<div class="flex items-center justify-center">
						<svg class="mr-2 h-5 w-5" viewBox="0 0 24 24">
							<path
								fill="#4285F4"
								d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
							/>
							<path
								fill="#34A853"
								d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
							/>
							<path
								fill="#FBBC05"
								d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
							/>
							<path
								fill="#EA4335"
								d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
							/>
						</svg>
						Continue with Google
					</div>
				</button>
			{/if}

			<div class="mt-4 space-y-2 text-center">
				<button
					onclick={onForgotPassword}
					class="text-sm text-blue-600 hover:text-blue-700 focus:outline-none"
				>
					Forgot your password?
				</button>

				<p class="text-sm text-gray-600">
					Don't have an account?
					<button
						onclick={onSwitchToRegister}
						class="text-blue-600 hover:text-blue-700 focus:outline-none"
					>
						Sign up
					</button>
				</p>
			</div>
		</div>
	</div>
{/if}
