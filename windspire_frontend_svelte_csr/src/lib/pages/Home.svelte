<script lang="ts">
	import { config } from '../config';
	import { route } from '@mateothegreat/svelte5-router';
	import { userStore } from '../stores/user';

	function handleSignIn() {
		// Dispatch a custom event that App.svelte can listen for
		window.dispatchEvent(new CustomEvent('open-login-modal'));
	}
</script>

<!-- Hero Section -->
<section class="bg-gradient-to-br from-blue-600 to-blue-800 py-20 text-white">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="text-center">
			<h1 class="mb-6 text-4xl font-bold md:text-6xl">
				Welcome to {config.APP_NAME}
			</h1>
			<p class="mb-8 text-xl text-blue-100 md:text-2xl">
				Modern boat management platform for marinas, yacht clubs, and boat owners
			</p>
			<div class="space-x-4">
				{#if $userStore}
					<!-- Logged in user buttons -->
					<a
						href="/boats"
						use:route
						class="rounded-lg bg-white px-8 py-3 font-semibold text-blue-600 transition-colors duration-200 hover:bg-blue-50"
					>
						View Boats
					</a>
					<a
						href="/users/{$userStore.id}"
						use:route
						class="rounded-lg border border-white px-8 py-3 font-semibold text-white transition-colors duration-200 hover:bg-white hover:text-blue-600"
					>
						My Profile
					</a>
				{:else}
					<!-- Not logged in buttons -->
					<button
						onclick={handleSignIn}
						class="rounded-lg bg-white px-8 py-3 font-semibold text-blue-600 transition-colors duration-200 hover:bg-blue-50"
					>
						Sign In
					</button>
					<a
						href="/auth"
						use:route
						class="rounded-lg border border-white px-8 py-3 font-semibold text-white transition-colors duration-200 hover:bg-white hover:text-blue-600"
					>
						Learn More
					</a>
				{/if}
			</div>
		</div>
	</div>
</section>

<!-- Features Section -->
<section class="bg-white py-16">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="mb-16 text-center">
			<h2 class="mb-4 text-3xl font-bold text-gray-900">
				Everything You Need to Manage Your Fleet
			</h2>
			<p class="mx-auto max-w-2xl text-lg text-gray-600">
				Comprehensive boat management solution with modern features designed for efficiency and ease
				of use.
			</p>
		</div>

		<div class="grid grid-cols-1 gap-8 md:grid-cols-3">
			<div class="p-6 text-center">
				<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
					<svg class="h-8 w-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 10V3L4 14h7v7l9-11h-7z"
						/>
					</svg>
				</div>
				<h3 class="mb-2 text-xl font-semibold text-gray-900">Fast & Reliable</h3>
				<p class="text-gray-600">
					Lightning-fast performance with reliable cloud infrastructure ensuring your data is always
					available.
				</p>
			</div>

			<div class="p-6 text-center">
				<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
					<svg class="h-8 w-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
						/>
					</svg>
				</div>
				<h3 class="mb-2 text-xl font-semibold text-gray-900">Secure</h3>
				<p class="text-gray-600">
					Enterprise-grade security with Firebase authentication and role-based access control.
				</p>
			</div>

			<div class="p-6 text-center">
				<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
					<svg class="h-8 w-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
						/>
					</svg>
				</div>
				<h3 class="mb-2 text-xl font-semibold text-gray-900">Analytics</h3>
				<p class="text-gray-600">
					Comprehensive reporting and analytics to help you make informed decisions about your
					fleet.
				</p>
			</div>
		</div>
	</div>
</section>

<!-- Call to Action Section -->
<section class="bg-gray-100 py-16">
	<div class="mx-auto max-w-7xl px-4 text-center sm:px-6 lg:px-8">
		{#if $userStore}
			<!-- Logged in user CTA -->
			<h2 class="mb-4 text-3xl font-bold text-gray-900">Welcome back, {$userStore.name}!</h2>
			<p class="mx-auto mb-8 max-w-2xl text-lg text-gray-600">
				Ready to manage your boats? View your fleet or explore all boats in the system.
			</p>
			<div class="space-x-4">
				<a
					href="/users/{$userStore.id}"
					use:route
					class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-colors duration-200 hover:bg-blue-700"
				>
					My Profile
				</a>
				<a
					href="/boats"
					use:route
					class="rounded-lg border border-blue-600 px-6 py-3 font-semibold text-blue-600 transition-colors duration-200 hover:bg-blue-600 hover:text-white"
				>
					View All Boats
				</a>
			</div>
		{:else}
			<!-- Not logged in CTA -->
			<h2 class="mb-4 text-3xl font-bold text-gray-900">Ready to Get Started?</h2>
			<p class="mx-auto mb-8 max-w-2xl text-lg text-gray-600">
				Join thousands of satisfied customers who trust {config.APP_NAME} for their boat management needs.
			</p>
			<div class="space-x-4">
				<button
					onclick={handleSignIn}
					class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-colors duration-200 hover:bg-blue-700"
				>
					Sign In
				</button>
				<a
					href="/auth"
					use:route
					class="rounded-lg border border-blue-600 px-6 py-3 font-semibold text-blue-600 transition-colors duration-200 hover:bg-blue-600 hover:text-white"
				>
					Learn More
				</a>
			</div>
		{/if}
	</div>
</section>
