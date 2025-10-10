<script lang="ts">
	import { route } from '@mateothegreat/svelte5-router';
</script>

<div class="mx-auto max-w-4xl px-4 py-16 text-center">
	<div class="mb-8">
		<svg
			class="mx-auto mb-4 h-24 w-24 text-gray-400"
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M9.172 16.172a4 4 0 015.656 0M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
			/>
		</svg>
		<h1 class="mb-4 text-4xl font-bold text-gray-900">Page Not Found</h1>
		<p class="mb-8 text-lg text-gray-600">Sorry, we couldn't find the page you're looking for.</p>
	</div>

	<div class="space-x-4">
		<a
			href="/"
			use:route
			class="rounded-lg bg-blue-600 px-6 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
		>
			Go Home
		</a>
		<a
			href="/boats"
			use:route
			class="rounded-lg border border-gray-300 px-6 py-3 font-medium text-gray-700 transition-colors duration-200 hover:bg-gray-50"
		>
			View Boats
		</a>
	</div>
</div>
