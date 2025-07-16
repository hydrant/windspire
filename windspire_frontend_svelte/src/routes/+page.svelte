<script lang="ts">
	// Hero section data
	const heroData = {
		title: "Welcome to Windspire",
		subtitle: "Empowering Innovation Through Technology",
		description: "Discover cutting-edge solutions that drive your business forward. We combine expertise with innovation to deliver exceptional results.",
		ctaText: "Get Started"
	};

	// Features data
	const features = [
		{
			title: "Innovation",
			description: "Cutting-edge technology solutions tailored to your needs",
			icon: "💡"
		},
		{
			title: "Reliability",
			description: "Dependable services you can trust for your business",
			icon: "🛡️"
		},
		{
			title: "Support",
			description: "24/7 customer support to help you succeed",
			icon: "🤝"
		}
	];
</script>

<svelte:head>
	<title>Windspire - Empowering Innovation Through Technology</title>
	<meta name="description" content="Discover cutting-edge solutions that drive your business forward with Windspire." />
</svelte:head>

<!-- Hero Section -->
<section class="bg-gradient-to-r from-blue-600 to-blue-800 text-white">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
		<div class="text-center">
			<h1 class="text-4xl md:text-6xl font-bold mb-6">
				{heroData.title}
			</h1>
			<p class="text-xl md:text-2xl mb-4 text-blue-100">
				{heroData.subtitle}
			</p>
			<p class="text-lg mb-8 text-blue-200 max-w-3xl mx-auto">
				{heroData.description}
			</p>
			<button class="bg-white text-blue-600 hover:bg-gray-100 font-semibold py-3 px-8 rounded-lg text-lg transition-colors duration-200 shadow-lg">
				{heroData.ctaText}
			</button>
		</div>
	</div>
</section>

<!-- Features Section -->
<section class="py-20 bg-white">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-16">
			<h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
				Why Choose Windspire?
			</h2>
			<p class="text-lg text-gray-600 max-w-2xl mx-auto">
				We deliver exceptional value through our core principles and commitment to excellence.
			</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-8">
			{#each features as feature}
				<div class="text-center p-6 rounded-lg hover:shadow-lg transition-shadow duration-200">
					<div class="text-4xl mb-4">{feature.icon}</div>
					<h3 class="text-xl font-semibold text-gray-900 mb-3">{feature.title}</h3>
					<p class="text-gray-600">{feature.description}</p>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Call to Action Section -->
<section class="bg-gray-100 py-16">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
		<h2 class="text-3xl font-bold text-gray-900 mb-4">
			Ready to Get Started?
		</h2>
		<p class="text-lg text-gray-600 mb-8 max-w-2xl mx-auto">
			Join thousands of satisfied customers who trust Windspire for their technology needs.
		</p>
		<div class="space-x-4">
			<button class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200">
				Contact Us
			</button>
			<button class="border border-blue-600 text-blue-600 hover:bg-blue-600 hover:text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200">
				Learn More
			</button>
		</div>
	</div>
</section>

<!-- Footer -->
<footer class="bg-gray-900 text-white py-12">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="grid grid-cols-1 md:grid-cols-4 gap-8">
			<div class="col-span-1 md:col-span-2">
				<div class="flex items-center mb-4">
					<svg class="h-8 w-8 text-blue-400" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
					</svg>
					<span class="ml-2 text-xl font-bold">Windspire</span>
				</div>
				<p class="text-gray-400 mb-4">
					Empowering innovation through technology. We deliver cutting-edge solutions that drive your business forward.
				</p>
			</div>
			
			<div>
				<h3 class="text-lg font-semibold mb-4">Quick Links</h3>
				<ul class="space-y-2 text-gray-400">
					<li><a href="/about" class="hover:text-white transition-colors">About</a></li>
					<li><a href="/services" class="hover:text-white transition-colors">Services</a></li>
					<li><a href="/contact" class="hover:text-white transition-colors">Contact</a></li>
				</ul>
			</div>
			
			<div>
				<h3 class="text-lg font-semibold mb-4">Contact Info</h3>
				<ul class="space-y-2 text-gray-400">
					<li>Email: info@windspire.com</li>
					<li>Phone: (555) 123-4567</li>
				</ul>
			</div>
		</div>
		
		<div class="border-t border-gray-800 mt-8 pt-8 text-center text-gray-400">
			<p>&copy; 2025 Windspire. All rights reserved.</p>
		</div>
	</div>
</footer>
