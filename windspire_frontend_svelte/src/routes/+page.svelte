<script lang="ts">
	// Hero section data
	const heroData = {
		title: 'Welcome to Windspire',
		subtitle: 'Empowering Innovation Through Technology',
		description:
			'Discover cutting-edge solutions that drive your business forward. We combine expertise with innovation to deliver exceptional results.',
		ctaText: 'Get Started'
	};

	// Features data
	const features = [
		{
			title: 'Innovation',
			description: 'Cutting-edge technology solutions tailored to your needs',
			icon: '💡'
		},
		{
			title: 'Reliability',
			description: 'Dependable services you can trust for your business',
			icon: '🛡️'
		},
		{
			title: 'Support',
			description: '24/7 customer support to help you succeed',
			icon: '🤝'
		}
	];
</script>

<svelte:head>
	<title>Windspire - Empowering Innovation Through Technology</title>
	<meta
		name="description"
		content="Discover cutting-edge solutions that drive your business forward with Windspire."
	/>
</svelte:head>

<!-- Hero Section -->
<section class="bg-gradient-to-r from-blue-600 to-blue-800 text-white">
	<div class="mx-auto max-w-7xl px-4 py-24 sm:px-6 lg:px-8">
		<div class="text-center">
			<h1 class="mb-6 text-4xl font-bold md:text-6xl">
				{heroData.title}
			</h1>
			<p class="mb-4 text-xl text-blue-100 md:text-2xl">
				{heroData.subtitle}
			</p>
			<p class="mx-auto mb-8 max-w-3xl text-lg text-blue-200">
				{heroData.description}
			</p>
			<button
				class="rounded-lg bg-white px-8 py-3 text-lg font-semibold text-blue-600 shadow-lg transition-colors duration-200 hover:bg-gray-100"
			>
				{heroData.ctaText}
			</button>
		</div>
	</div>
</section>

<!-- Features Section -->
<section class="bg-white py-20">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="mb-16 text-center">
			<h2 class="mb-4 text-3xl font-bold text-gray-900 md:text-4xl">Why Choose Windspire?</h2>
			<p class="mx-auto max-w-2xl text-lg text-gray-600">
				We deliver exceptional value through our core principles and commitment to excellence.
			</p>
		</div>

		<div class="grid grid-cols-1 gap-8 md:grid-cols-3">
			{#each features as feature (feature.title)}
				<div class="rounded-lg p-6 text-center transition-shadow duration-200 hover:shadow-lg">
					<div class="mb-4 text-4xl">{feature.icon}</div>
					<h3 class="mb-3 text-xl font-semibold text-gray-900">{feature.title}</h3>
					<p class="text-gray-600">{feature.description}</p>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Call to Action Section -->
<section class="bg-gray-100 py-16">
	<div class="mx-auto max-w-7xl px-4 text-center sm:px-6 lg:px-8">
		<h2 class="mb-4 text-3xl font-bold text-gray-900">Ready to Get Started?</h2>
		<p class="mx-auto mb-8 max-w-2xl text-lg text-gray-600">
			Join thousands of satisfied customers who trust Windspire for their technology needs.
		</p>
		<div class="space-x-4">
			<button
				class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-colors duration-200 hover:bg-blue-700"
			>
				Contact Us
			</button>
			<button
				class="rounded-lg border border-blue-600 px-6 py-3 font-semibold text-blue-600 transition-colors duration-200 hover:bg-blue-600 hover:text-white"
			>
				Learn More
			</button>
		</div>
	</div>
</section>

<!-- Footer -->
<footer class="bg-gray-900 py-12 text-white">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="grid grid-cols-1 gap-8 md:grid-cols-4">
			<div class="col-span-1 md:col-span-2">
				<div class="mb-4 flex items-center">
					<svg class="h-8 w-8 text-blue-400" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
					</svg>
					<span class="ml-2 text-xl font-bold">Windspire</span>
				</div>
				<p class="mb-4 text-gray-400">
					Empowering innovation through technology. We deliver cutting-edge solutions that drive
					your business forward.
				</p>
			</div>

			<div>
				<h3 class="mb-4 text-lg font-semibold">Quick Links</h3>
				<ul class="space-y-2 text-gray-400">
					<li><a href="/about" class="transition-colors hover:text-white">About</a></li>
					<li><a href="/services" class="transition-colors hover:text-white">Services</a></li>
					<li><a href="/contact" class="transition-colors hover:text-white">Contact</a></li>
				</ul>
			</div>

			<div>
				<h3 class="mb-4 text-lg font-semibold">Contact Info</h3>
				<ul class="space-y-2 text-gray-400">
					<li>Email: info@windspire.com</li>
					<li>Phone: (555) 123-4567</li>
				</ul>
			</div>
		</div>

		<div class="mt-8 border-t border-gray-800 pt-8 text-center text-gray-400">
			<p>&copy; 2025 Windspire. All rights reserved.</p>
		</div>
	</div>
</footer>
