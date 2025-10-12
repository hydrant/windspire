<script lang="ts">
	import { Router, type RouteConfig, type RouterInstance } from '@mateothegreat/svelte5-router';

	// Import pages
	import Home from './lib/pages/Home.svelte';
	import Boats from './lib/pages/Boats.svelte';
	import UserProfile from './lib/pages/UserProfile.svelte';
	// @ts-expect-error - Svelte component imports
	import TestProfile from './lib/pages/TestProfile.svelte';
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
			path: '/users/*',
			component: UserProfile,
			hooks: {
				pre: () => {
					console.log('Pre-hook for /users/* triggered!');
					// Check if token exists in localStorage instead of relying on userStore
					const token = localStorage.getItem('windspire_token');
					if (!token) {
						// Redirect to home instead of showing login modal
						window.location.hash = '/';
						return false; // Prevent navigation
					}
					return true; // Allow navigation
				}
			}
		},
		{
			path: '/boats',
			component: Boats,
			hooks: {
				pre: () => {
					// Check if user is authenticated
					const token = localStorage.getItem('windspire_token');
					if (!token) {
						// Redirect to home instead of showing login modal
						window.location.hash = '/';
						return false; // Prevent navigation
					}
					return true; // Allow navigation
				}
			}
		},
		{
			path: '/test-profile',
			component: TestProfile,
			hooks: {
				pre: () => {
					console.log('Pre-hook for /test-profile, userStore:', $userStore);
					// Check if token exists in localStorage instead of relying on userStore
					const token = localStorage.getItem('windspire_token');
					console.log('Token in pre-hook:', !!token);
					if (!token) {
						console.log('No token found, redirecting to home');
						window.location.hash = '/';
						return false; // Prevent navigation
					}
					console.log('Token found, allowing navigation');
					return true; // Allow navigation
				}
			}
		},
		{
			path: '/test-user-route',
			component: UserProfile,
			hooks: {
				pre: () => {
					console.log('Pre-hook for /test-user-route triggered!');
					return true;
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
		console.log('App mounted, current URL:', window.location.href);
		console.log('Current pathname:', window.location.pathname);
		console.log('Routes configured:', routes);
		console.log(
			'Routes detailed:',
			JSON.stringify(
				routes.map((r) => ({ path: r.path, component: r.component?.name || 'unnamed' })),
				null,
				2
			)
		);
		const token = localStorage.getItem('windspire_token');
		console.log('Token in localStorage:', !!token);
		console.log('User store value:', $userStore);
		if (token) {
			try {
				const response = await fetch(`${config.API_BASE_URL}/auth/me`, {
					headers: {
						Authorization: `Bearer ${token}`
					}
				});

				if (response.ok) {
					const userData = await response.json();
					console.log('Auth response userData:', userData);
					if (userData.success && userData.data) {
						console.log('Setting userStore with data:', userData.data);
						userStore.set(userData.data);
						console.log('UserStore after setting:', $userStore);
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

		// Listen for login modal requests from other components
		window.addEventListener('open-login-modal', openLoginModal);

		return () => {
			window.removeEventListener('auth-changed', handleAuthChange);
			window.removeEventListener('open-login-modal', openLoginModal);
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
