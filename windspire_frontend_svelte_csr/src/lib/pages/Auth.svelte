<div class="mx-auto max-w-4xl px-4 py-8">
	<h1 class="mb-8 text-3xl font-bold text-gray-900">Authentication</h1>

	<div class="rounded-lg bg-white p-8 shadow-md">
		<h2 class="mb-6 text-xl font-semibold text-gray-900">Sign In to Windspire</h2>

		<div class="space-y-4">
			<button
				class="w-full rounded-lg bg-blue-600 px-4 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
			>
				Sign in with Google
			</button>

			<div class="relative">
				<div class="absolute inset-0 flex items-center">
					<div class="w-full border-t border-gray-300"></div>
				</div>
				<div class="relative flex justify-center text-sm">
					<span class="bg-white px-2 text-gray-500">Or sign in with email</span>
				</div>
			</div>

			<form class="space-y-4">
				<div>
					<label for="email" class="mb-1 block text-sm font-medium text-gray-700">Email</label>
					<input
						type="email"
						id="email"
						class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
						placeholder="your.email@example.com"
					/>
				</div>

				<div>
					<label for="password" class="mb-1 block text-sm font-medium text-gray-700">Password</label
					>
					<input
						type="password"
						id="password"
						class="w-full rounded-lg border border-gray-300 px-3 py-2 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
						placeholder="••••••••"
					/>
				</div>

				<button
					type="submit"
					class="w-full rounded-lg bg-blue-600 px-4 py-3 font-medium text-white transition-colors duration-200 hover:bg-blue-700"
				>
					Sign In
				</button>
			</form>
		</div>
	</div>
</div>
