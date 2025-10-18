<script lang="ts">
	import '../app.css';
	import Navigation from '$lib/components/Navigation.svelte';
	import LoginModal from '$lib/components/LoginModal.svelte';
	import { firebaseAuth } from '$lib/firebase/auth';
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
						Authorization: `Bearer ${token}`
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
			} catch {
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

	async function handleGoogleSignIn() {
		try {
			console.log('Starting Firebase Google sign-in...');

			// Sign in with Google using Firebase
			const result = await firebaseAuth.signInWithGoogle();
			console.log('Firebase Google sign-in successful:', result);

			if (result) {
				// Get ID token and send to backend
				const idToken = await result.user.getIdToken();
				console.log('Got Firebase ID token, sending to backend...');

				// Send token to backend for verification and session creation
				const response = await fetch('http://localhost:8080/auth/firebase', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ id_token: idToken })
				});

				if (response.ok) {
					const data = await response.json();
					console.log('Backend verification successful:', data);

					if (data.success && data.data) {
						// Store the JWT token from backend
						localStorage.setItem('windspire_token', data.data.token);

						// Trigger auth change event
						window.dispatchEvent(new CustomEvent('auth-changed'));

						closeLoginModal();
						// Refresh user state
						checkAuthStatus();
					} else {
						throw new Error(data.message || 'Authentication failed');
					}
				} else {
					const errorText = await response.text();
					console.error('Backend verification failed:', response.status, errorText);
					alert(`Authentication verification failed: ${response.status} - ${errorText}`);
				}
			}
		} catch (error: unknown) {
			console.error('Google sign-in error:', error);

			// Show user-friendly error message
			let errorMessage = 'Google sign-in failed. ';
			if (error && typeof error === 'object' && 'message' in error) {
				errorMessage += (error as { message: string }).message;
			} else {
				errorMessage += 'Please try again.';
			}
			alert(errorMessage);
			closeLoginModal();
		}
	}

	async function handleLogin() {
		// Firebase email/password authentication is handled in the LoginForm component
		// This callback is triggered after successful login to refresh the UI state
		console.log('Login successful, refreshing auth status...');
		checkAuthStatus();
	}

	function openLoginModal() {
		console.log('Opening login modal...');
		isLoginModalOpen = true;
	}

	async function handleLogout() {
		try {
			const token = localStorage.getItem('windspire_token');
			if (token) {
				await fetch('http://localhost:8080/auth/logout', {
					method: 'POST',
					headers: {
						Authorization: `Bearer ${token}`,
						'Content-Type': 'application/json'
					}
				});
			}

			// Also sign out from Firebase to clear any auth state
			await firebaseAuth.signOut();
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
		onGoogleSignIn={handleGoogleSignIn}
	/>
</div>
