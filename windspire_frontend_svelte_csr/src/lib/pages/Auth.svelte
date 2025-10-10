<script lang="ts">
	import { firebaseAuthService } from '../firebase';
	import { userStore } from '../stores/user';
	import { config } from '../config';
	import { route } from '@mateothegreat/svelte5-router';

	type AuthMode = 'signin' | 'signup';
	let authMode: AuthMode = $state('signin');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let displayName = $state('');
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let successMessage = $state<string | null>(null);

	function switchMode(mode: AuthMode) {
		authMode = mode;
		error = null;
		successMessage = null;
		// Clear form fields when switching modes
		email = '';
		password = '';
		confirmPassword = '';
		displayName = '';
	}

	async function signInWithGoogle() {
		isLoading = true;
		error = null;

		try {
			const result = await firebaseAuthService.signInWithGoogle();
			if (result && result.user) {
				// Get Firebase ID token
				const idToken = await result.user.getIdToken();

				// Include display name if available
				const requestBody: any = { id_token: idToken };
				if (result.user.displayName) {
					requestBody.display_name = result.user.displayName;
				}

				// Send to our backend for verification and JWT generation
				const response = await fetch(`${config.API_BASE_URL}/auth/firebase`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify(requestBody)
				});

				if (response.ok) {
					const data = await response.json();
					if (data.success && data.data) {
						// Store our backend JWT token
						localStorage.setItem('windspire_token', data.data.token);
						userStore.set(data.data.user);

						// Dispatch auth change event
						window.dispatchEvent(new CustomEvent('auth-changed'));

						// Navigate to home
						window.location.hash = '/';
					} else {
						throw new Error(data.message || 'Authentication failed');
					}
				} else {
					throw new Error('Backend authentication failed');
				}
			}
		} catch (err: any) {
			console.error('Google sign-in error:', err);
			error = err.message || 'Failed to sign in with Google';
		} finally {
			isLoading = false;
		}
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		error = null;
		successMessage = null;

		// Validation
		if (!email || !password) {
			error = 'Please fill in all required fields';
			return;
		}

		if (authMode === 'signup') {
			if (!displayName) {
				error = 'Please enter your display name';
				return;
			}
			if (password !== confirmPassword) {
				error = 'Passwords do not match';
				return;
			}
			if (password.length < 6) {
				error = 'Password should be at least 6 characters long';
				return;
			}
		}

		isLoading = true;

		try {
			let result;
			if (authMode === 'signin') {
				result = await firebaseAuthService.signInWithEmailAndPassword(email, password);
			} else {
				result = await firebaseAuthService.signUpWithEmailAndPassword(email, password, displayName);
			}

			if (result && result.user) {
				// Get Firebase ID token
				const idToken = await result.user.getIdToken();

				// Include display name for both sign-in and sign-up
				const requestBody: any = { id_token: idToken };
				if (authMode === 'signup' && displayName) {
					requestBody.display_name = displayName;
				} else if (result.user.displayName) {
					requestBody.display_name = result.user.displayName;
				}

				// Send to our backend for verification and JWT generation
				const response = await fetch(`${config.API_BASE_URL}/auth/firebase`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify(requestBody)
				});

				if (response.ok) {
					const data = await response.json();
					if (data.success && data.data) {
						// Store our backend JWT token
						localStorage.setItem('windspire_token', data.data.token);
						userStore.set(data.data.user);

						// Dispatch auth change event
						window.dispatchEvent(new CustomEvent('auth-changed'));

						if (authMode === 'signup') {
							successMessage = 'Account created successfully! Redirecting...';
							setTimeout(() => {
								window.location.hash = '/';
							}, 2000);
						} else {
							// Navigate to home
							window.location.hash = '/';
						}
					} else {
						throw new Error(data.message || 'Authentication failed');
					}
				} else {
					throw new Error('Backend authentication failed');
				}
			}
		} catch (err: any) {
			console.error(`${authMode} error:`, err);
			error = err.message || `Failed to ${authMode === 'signin' ? 'sign in' : 'create account'}`;
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="mx-auto max-w-4xl px-4 py-8">
	<h1 class="mb-8 text-3xl font-bold text-gray-900">Authentication</h1>

	<div class="rounded-lg bg-white p-8 shadow-md">
		<!-- Mode switcher -->
		<div class="mb-6 flex justify-center">
			<div class="inline-flex rounded-lg bg-gray-100 p-1">
				<button
					onclick={() => switchMode('signin')}
					class="rounded-md px-4 py-2 text-sm font-medium transition-colors duration-200"
					class:bg-white={authMode === 'signin'}
					class:text-blue-600={authMode === 'signin'}
					class:shadow-sm={authMode === 'signin'}
					class:text-gray-500={authMode !== 'signin'}
				>
					Sign In
				</button>
				<button
					onclick={() => switchMode('signup')}
					class="rounded-md px-4 py-2 text-sm font-medium transition-colors duration-200"
					class:bg-white={authMode === 'signup'}
					class:text-blue-600={authMode === 'signup'}
					class:shadow-sm={authMode === 'signup'}
					class:text-gray-500={authMode !== 'signup'}
				>
					Sign Up
				</button>
			</div>
		</div>

		<h2 class="mb-6 text-xl font-semibold text-gray-900">
			{authMode === 'signin' ? 'Sign In to Windspire' : 'Create Your Windspire Account'}
		</h2>

		{#if error}
			<div class="mb-4 rounded-md bg-red-50 p-3">
				<p class="text-sm text-red-700">{error}</p>
			</div>
		{/if}

		{#if successMessage}
			<div class="mb-4 rounded-md bg-green-50 p-3">
				<p class="text-sm text-green-700">{successMessage}</p>
			</div>
		{/if}

		<div class="space-y-4">
			<button
				onclick={signInWithGoogle}
				disabled={isLoading}
				class="w-full rounded-lg bg-blue-600 px-4 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-gray-400"
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
						Processing...
					</div>
				{:else}
					{authMode === 'signin' ? 'Sign in with Google' : 'Sign up with Google'}
				{/if}
			</button>

			<div class="relative">
				<div class="absolute inset-0 flex items-center">
					<div class="w-full border-t border-gray-300"></div>
				</div>
				<div class="relative flex justify-center text-sm">
					<span class="bg-white px-2 text-gray-500">
						Or {authMode === 'signin' ? 'sign in' : 'sign up'} with email
					</span>
				</div>
			</div>

			<form onsubmit={handleSubmit} class="space-y-4">
				{#if authMode === 'signup'}
					<div>
						<label for="displayName" class="mb-1 block text-sm font-medium text-gray-700">
							Display Name
						</label>
						<input
							type="text"
							id="displayName"
							bind:value={displayName}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
							placeholder="Your full name"
							required
						/>
					</div>
				{/if}

				<div>
					<label for="email" class="mb-1 block text-sm font-medium text-gray-700">Email</label>
					<input
						type="email"
						id="email"
						bind:value={email}
						class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
						placeholder="your.email@example.com"
						required
					/>
				</div>

				<div>
					<label for="password" class="mb-1 block text-sm font-medium text-gray-700">Password</label
					>
					<input
						type="password"
						id="password"
						bind:value={password}
						class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
						placeholder="••••••••"
						required
					/>
				</div>

				{#if authMode === 'signup'}
					<div>
						<label for="confirmPassword" class="mb-1 block text-sm font-medium text-gray-700">
							Confirm Password
						</label>
						<input
							type="password"
							id="confirmPassword"
							bind:value={confirmPassword}
							class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
							placeholder="••••••••"
							required
						/>
					</div>
				{/if}

				<button
					type="submit"
					disabled={isLoading}
					class="w-full rounded-lg bg-blue-600 px-4 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-gray-400"
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
							Processing...
						</div>
					{:else}
						{authMode === 'signin' ? 'Sign In' : 'Create Account'}
					{/if}
				</button>
			</form>
		</div>
	</div>
</div>
