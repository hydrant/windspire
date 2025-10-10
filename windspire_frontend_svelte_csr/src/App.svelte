<script lang="ts">
	import { Router, type RouteConfig, type RouterInstance } from '@mateothegreat/svelte5-router';

	// Import pages
	import Home from './lib/pages/Home.svelte';
	import Boats from './lib/pages/Boats.svelte';
	// @ts-ignore - Svelte component imports
	import Auth from './lib/pages/Auth.svelte';
	import NotFound from './lib/pages/NotFound.svelte';

	// Import layout components
	import Navigation from './lib/components/Navigation.svelte';
	import LoginModal from './lib/components/LoginModal.svelte';

	// Import stores and auth
	import { userStore } from './lib/stores/user';
	import { onMount } from 'svelte';
	import { config } from './lib/config';

	let isLoginModalOpen = $state(false);
	let router = $state<RouterInstance>();

	// Route configuration
	const routes: RouteConfig[] = [
		{
			path: '/boats',
			component: Boats,
			hooks: {
				pre: () => {
					// Check if user is authenticated
					if (!$userStore) {
						isLoginModalOpen = true;
						return false; // Prevent navigation
					}
					return true; // Allow navigation
				}
			}
		},
		{
			path: '/auth',
			component: Auth
		},
		{
			path: '/',
			component: Home
		},
		{
			component: NotFound
		}
	];

	// Check authentication status on app load
	onMount(async () => {
		const token = localStorage.getItem('windspire_token');
		if (token) {
			try {
				const response = await fetch(`${config.API_BASE_URL}/auth/me`, {
					headers: {
						Authorization: `Bearer ${token}`
					}
				});

				if (response.ok) {
					const userData = await response.json();
					if (userData.success && userData.data) {
						userStore.set(userData.data);
					}
				} else {
					// Token invalid, remove it
					localStorage.removeItem('windspire_token');
				}
			} catch (error) {
				console.error('Error checking auth status:', error);
				localStorage.removeItem('windspire_token');
			}
		}
	});

	// Listen for auth changes
	function handleAuthChange() {
		const token = localStorage.getItem('windspire_token');
		if (!token) {
			userStore.set(null);
		}
	}

	onMount(() => {
		window.addEventListener('auth-changed', handleAuthChange);
		return () => {
			window.removeEventListener('auth-changed', handleAuthChange);
		};
	});

	function openLoginModal() {
		isLoginModalOpen = true;
	}

	function closeLoginModal() {
		isLoginModalOpen = false;
	}
</script>

<Navigation {openLoginModal} />

<main class="min-h-screen">
	<Router {routes} bind:instance={router} />
</main>

{#if isLoginModalOpen}
	<LoginModal {closeLoginModal} />
{/if}
