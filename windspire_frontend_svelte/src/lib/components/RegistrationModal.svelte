<script lang="ts">
	import { firebaseAuth } from '$lib/firebase/auth';

	interface Props {
		isOpen: boolean;
		onClose: () => void;
		onSuccess: () => void;
		onSwitchToLogin: () => void;
	}

	const { isOpen, onClose, onSuccess, onSwitchToLogin }: Props = $props();

	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let displayName = $state('');
	let isLoading = $state(false);
	let error = $state('');
	let showSuccess = $state(false);

	function resetForm() {
		email = '';
		password = '';
		confirmPassword = '';
		displayName = '';
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

		// Validation
		if (!email || !password || !displayName) {
			error = 'Please fill in all fields';
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

		isLoading = true;

		try {
			console.log('Starting Firebase registration...');

			// Create user with email/password
			const result = await firebaseAuth.signUpWithEmail(email, password, displayName);
			console.log('Firebase registration successful, sending to backend...');

			// Get ID token and send to backend with display name
			const idToken = await result.user.getIdToken();

			console.log('Sending to backend - display_name:', displayName);
			console.log('ID token created, sending registration request...');

			const response = await fetch('http://localhost:8080/auth/firebase', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					id_token: idToken,
					display_name: displayName // Send display name separately
				})
			});

			if (response.ok) {
				const data = await response.json();
				if (data.success && data.data) {
					// Store the JWT token from backend
					localStorage.setItem('windspire_token', data.data.token);

					// Trigger auth change event
					window.dispatchEvent(new CustomEvent('auth-changed'));

					console.log('Registration successful');
					showSuccess = true;

					// Show success message for a moment, then close
					setTimeout(() => {
						handleClose();
						onSuccess();
					}, 2000);
				} else {
					throw new Error(data.message || 'Registration failed');
				}
			} else {
				const errorText = await response.text();
				throw new Error(`Backend registration failed: ${response.status} ${errorText}`);
			}
		} catch (err: unknown) {
			console.error('Registration error:', err);
			error = err instanceof Error ? err.message : 'Registration failed. Please try again.';
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
					<h3 class="text-lg font-medium text-gray-900">Account Created!</h3>
					<p class="mt-2 text-sm text-gray-500">
						Please check your email to verify your account. A verification email has been sent to {email}.
					</p>
				</div>
			{:else}
				<!-- Registration form -->
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-xl font-medium text-gray-900">Create Account</h3>
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

				{#if error}
					<div class="mb-4 rounded-md bg-red-50 p-3">
						<p class="text-sm text-red-700">{error}</p>
					</div>
				{/if}

				<form onsubmit={handleSubmit} class="space-y-4">
					<div>
						<label for="displayName" class="block text-sm font-medium text-gray-700"
							>Full Name</label
						>
						<input
							type="text"
							id="displayName"
							bind:value={displayName}
							class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
							placeholder="Enter your full name"
							required
						/>
					</div>

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

					<div>
						<label for="confirmPassword" class="block text-sm font-medium text-gray-700"
							>Confirm Password</label
						>
						<input
							type="password"
							id="confirmPassword"
							bind:value={confirmPassword}
							class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
							placeholder="Confirm your password"
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
								Creating Account...
							</div>
						{:else}
							Create Account
						{/if}
					</button>
				</form>

				<div class="mt-4 text-center">
					<p class="text-sm text-gray-600">
						Already have an account?
						<button
							onclick={onSwitchToLogin}
							class="text-blue-600 hover:text-blue-700 focus:outline-none"
						>
							Sign in
						</button>
					</p>
				</div>
			{/if}
		</div>
	</div>
{/if}
