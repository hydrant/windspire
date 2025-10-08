<script lang="ts">
	import LoginForm from './LoginForm.svelte';
	import RegistrationModal from './RegistrationModal.svelte';
	import PasswordResetModal from './PasswordResetModal.svelte';

	interface Props {
		isOpen: boolean;
		onClose?: () => void;
		onLogin?: () => void;
		onGoogleSignIn?: () => void;
	}

	const { isOpen, onClose, onLogin, onGoogleSignIn }: Props = $props();

	type AuthView = 'login' | 'register' | 'reset-password';
	let currentView: AuthView = $state('login');

	// Reset view when modal opens/closes
	$effect(() => {
		if (isOpen) {
			currentView = 'login';
		}
	});

	function handleClose() {
		currentView = 'login';
		onClose?.();
	}

	function handleSwitchToRegister() {
		currentView = 'register';
	}

	function handleSwitchToLogin() {
		currentView = 'login';
	}

	function handleForgotPassword() {
		currentView = 'reset-password';
	}

	async function handleGoogleSignIn() {
		// Handle Google Sign-In - delegate to parent
		onGoogleSignIn?.();
	}

	async function handleLoginSuccess() {
		// User successfully logged in, close modal and notify parent
		handleClose();
		onLogin?.();
	}

	async function handleRegistrationSuccess() {
		// Registration successful, user should verify email
		// Keep modal open to show success message
	}
</script>

{#if isOpen}
	{#if currentView === 'login'}
		<LoginForm
			{isOpen}
			onClose={handleClose}
			onSuccess={handleLoginSuccess}
			onSwitchToRegister={handleSwitchToRegister}
			onForgotPassword={handleForgotPassword}
			onGoogleSignIn={handleGoogleSignIn}
		/>
	{:else if currentView === 'register'}
		<RegistrationModal
			{isOpen}
			onClose={handleClose}
			onSuccess={handleRegistrationSuccess}
			onSwitchToLogin={handleSwitchToLogin}
		/>
	{:else if currentView === 'reset-password'}
		<PasswordResetModal {isOpen} onClose={handleClose} onBackToLogin={handleSwitchToLogin} />
	{/if}
{/if}
