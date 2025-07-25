<script lang="ts">
	import '../app.css';
	import Navigation from '$lib/components/Navigation.svelte';
	import LoginModal from '$lib/components/LoginModal.svelte';
	import { onMount } from 'svelte';
	import { userStore } from '$lib/stores/user';

	let { children } = $props();

	// User state - will be populated from JWT token
	let user = $state<{ id: string; name: string; email: string; picture?: string } | null>(null);
	let isLoginModalOpen = $state(false);

	// Check for existing JWT token and get user info from backend
	async function checkAuthStatus() {
		const token = localStorage.getItem('windspire_token');
		if (token) {
			try {
				// Call the /auth/me endpoint to get current user info
				const response = await fetch('http://localhost:8080/auth/me', {
					headers: {
						'Authorization': `Bearer ${token}`
					}
				});
				
				if (response.ok) {
					const userData = await response.json();
					// Handle wrapped response from backend
					const userInfo = userData.data || userData;
					console.log('User info from /auth/me:', userInfo);
					console.log('Picture field:', userInfo.picture);
					user = {
						id: userInfo.id,
						name: userInfo.name,
						email: userInfo.email,
						picture: userInfo.picture
					};
					// Update the store
					userStore.set(user);
				} else {
					// Token invalid or expired, remove it
					localStorage.removeItem('windspire_token');
					user = null;
					userStore.set(null);
				}
			} catch (error) {
				console.log('Failed to get user info, removing token');
				localStorage.removeItem('windspire_token');
				user = null;
				userStore.set(null);
			}
		}
	}

	// Initialize auth check and listen for auth changes
	onMount(() => {
		checkAuthStatus();
		
		// Listen for custom auth-changed events
		const handleAuthChange = () => {
			checkAuthStatus();
		};
		
		// Listen for storage changes (cross-tab)
		const handleStorageChange = (e: StorageEvent) => {
			if (e.key === 'windspire_token') {
				checkAuthStatus();
			}
		};
		
		window.addEventListener('auth-changed', handleAuthChange);
		window.addEventListener('storage', handleStorageChange);
		
		// Also check periodically for token expiration (every 5 minutes)
		const interval = setInterval(checkAuthStatus, 300000); // 5 minutes
		
		return () => {
			window.removeEventListener('auth-changed', handleAuthChange);
			window.removeEventListener('storage', handleStorageChange);
			clearInterval(interval);
		};
	});

	async function handleLogin() {
		try {
			console.log('Testing backend connectivity...');
			
			// First test if backend is reachable
			try {
				const healthResponse = await fetch('http://localhost:8080/health');
				console.log('Health check response:', healthResponse.status);
				if (!healthResponse.ok) {
					throw new Error(`Health check failed: ${healthResponse.status}`);
				}
			} catch (healthError) {
				console.error('Backend health check failed:', healthError);
				closeLoginModal();
				alert('Backend server is not accessible. Please ensure the backend is running on port 8080.');
				return;
			}

			console.log('Backend is accessible, attempting login...');
			
			// Get the authorization URL from backend
			const response = await fetch('http://localhost:8080/auth/login');
			console.log('Login response status:', response.status);
			console.log('Login response headers:', Object.fromEntries(response.headers.entries()));
			
			if (response.ok) {
				const data = await response.json();
				console.log('Login response data:', data);
				
				// Check if response is wrapped in success/data structure
				const authData = data.data || data;
				
				if (authData.authorization_url) {
					// Redirect to Google OAuth
					window.location.href = authData.authorization_url;
				} else {
					console.error('No authorization_url in response:', data);
					console.error('Expected structure: { data: { authorization_url: "..." } } or { authorization_url: "..." }');
					closeLoginModal();
					alert('Invalid response from login service. Check backend logs for OAuth configuration.');
				}
			} else {
				const errorText = await response.text();
				console.error('Login failed:', response.status, response.statusText, errorText);
				closeLoginModal();
				alert(`Login service error: ${response.status} ${response.statusText}`);
			}
		} catch (error) {
			console.error('Login failed:', error);
			closeLoginModal();
			const errorMessage = error instanceof Error ? error.message : 'Unknown error';
			alert(`Unable to connect to login service: ${errorMessage}`);
		}
	}

	function openLoginModal() {
		isLoginModalOpen = true;
	}

	async function handleLogout() {
		try {
			const token = localStorage.getItem('windspire_token');
			if (token) {
				await fetch('http://localhost:8080/auth/logout', {
					method: 'POST',
					headers: {
						'Authorization': `Bearer ${token}`,
						'Content-Type': 'application/json'
					}
				});
			}
		} catch (error) {
			console.error('Logout failed:', error);
		} finally {
			// Always clear local state
			localStorage.removeItem('windspire_token');
			user = null;
			userStore.set(null);
			// Trigger auth change event
			window.dispatchEvent(new CustomEvent('auth-changed'));
		}
	}

	function closeLoginModal() {
		isLoginModalOpen = false;
	}
</script>

<div class="min-h-screen bg-gray-50">
	<Navigation {user} onLogin={openLoginModal} onLogout={handleLogout} />
	
	<main>
		{@render children()}
	</main>

	<LoginModal
		isOpen={isLoginModalOpen}
		onClose={closeLoginModal}
		onLogin={handleLogin}
	/>
</div>
