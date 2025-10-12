<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		try {
			// Get the JWT token from URL params (sent by backend redirect)
			const urlParams = new URLSearchParams(window.location.search);
			const token = urlParams.get('token');

			if (!token) {
				error = 'No authentication token received.';
				loading = false;
				return;
			}

			// Store the JWT token
			localStorage.setItem('windspire_token', token);

			// Trigger a custom event to notify the layout of the auth change
			window.dispatchEvent(new CustomEvent('auth-changed'));

			// Redirect to home page
			await goto('/');
		} catch {
			error = 'An error occurred during authentication.';
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Authenticating - Windspire</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-gray-50">
	<div class="w-full max-w-md space-y-8 text-center">
		{#if loading}
			<div>
				<div class="mx-auto h-12 w-12 text-blue-600">
					<svg
						class="h-12 w-12 animate-spin"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
				</div>
				<h2 class="mt-6 text-3xl font-extrabold text-gray-900">Signing you in...</h2>
				<p class="mt-2 text-sm text-gray-600">Please wait while we complete your authentication.</p>
			</div>
		{:else if error}
			<div>
				<div class="mx-auto h-12 w-12 text-red-600">
					<svg
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
						></path>
					</svg>
				</div>
				<h2 class="mt-6 text-3xl font-extrabold text-gray-900">Authentication Failed</h2>
				<p class="mt-2 text-sm text-gray-600">
					{error}
				</p>
				<div class="mt-6">
					<a
						href="/"
						class="inline-flex items-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
					>
						Return to Home
					</a>
				</div>
			</div>
		{/if}
	</div>
</div>
