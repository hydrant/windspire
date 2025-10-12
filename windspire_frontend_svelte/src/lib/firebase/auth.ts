import {
	signInWithPopup,
	signOut,
	GoogleAuthProvider,
	createUserWithEmailAndPassword,
	signInWithEmailAndPassword,
	sendPasswordResetEmail,
	sendEmailVerification,
	updateProfile,
	type User,
	type UserCredential,
	onAuthStateChanged,
	type Unsubscribe
} from 'firebase/auth';
import { auth } from './config';
import { browser } from '$app/environment';

export class FirebaseAuthService {
	private static instance: FirebaseAuthService | null = null;

	public static getInstance(): FirebaseAuthService {
		if (!FirebaseAuthService.instance) {
			FirebaseAuthService.instance = new FirebaseAuthService();
		}
		return FirebaseAuthService.instance;
	}

	/**
	 * Sign in with Google using Firebase Auth
	 */
	async signInWithGoogle(): Promise<UserCredential | null> {
		if (!browser || !auth) {
			console.warn(
				'Firebase auth not available - either not in browser environment or Firebase not properly initialized'
			);
			throw new Error(
				'Firebase authentication is not available. Please check your Firebase configuration.'
			);
		}

		try {
			const provider = new GoogleAuthProvider();
			// Request additional scopes if needed
			provider.addScope('email');
			provider.addScope('profile');

			const result = await signInWithPopup(auth, provider);
			return result;
		} catch (error: any) {
			console.error('Firebase Google sign-in error:', error);

			// Provide specific guidance for common errors
			if (error.code === 'auth/operation-not-allowed') {
				throw new Error(
					'Google Sign-In is not enabled in Firebase. Please enable it in the Firebase Console under Authentication > Sign-in method > Google.'
				);
			} else if (error.code === 'auth/unauthorized-domain') {
				throw new Error(
					'This domain is not authorized for OAuth operations. Please add your domain to the Firebase Console under Authentication > Settings > Authorized domains.'
				);
			} else if (error.code === 'auth/popup-blocked') {
				throw new Error(
					'Sign-in popup was blocked by the browser. Please allow popups and try again.'
				);
			} else if (error.code === 'auth/popup-closed-by-user') {
				throw new Error('Sign-in was cancelled. Please try again.');
			}

			throw error;
		}
	}

	/**
	 * Sign up with email and password
	 */
	async signUpWithEmail(
		email: string,
		password: string,
		displayName?: string
	): Promise<UserCredential> {
		if (!browser || !auth) {
			throw new Error(
				'Firebase authentication is not available. Please check your Firebase configuration.'
			);
		}

		try {
			const result = await createUserWithEmailAndPassword(auth, email, password);

			// Update display name if provided
			if (displayName && result.user) {
				await updateProfile(result.user, { displayName });
				console.log('Updated Firebase profile with displayName:', displayName);

				// Refresh the user to get updated token with display name
				await result.user.reload();
				console.log('Reloaded user profile, displayName now:', result.user.displayName);
			}

			// Send email verification
			if (result.user) {
				await sendEmailVerification(result.user);
			}

			return result;
		} catch (error: any) {
			console.error('Firebase email sign-up error:', error);

			// Provide specific guidance for common errors
			if (error.code === 'auth/email-already-in-use') {
				throw new Error('An account with this email already exists. Please sign in instead.');
			} else if (error.code === 'auth/invalid-email') {
				throw new Error('Please enter a valid email address.');
			} else if (error.code === 'auth/weak-password') {
				throw new Error('Password should be at least 6 characters long.');
			} else if (error.code === 'auth/operation-not-allowed') {
				throw new Error(
					'Email/password authentication is not enabled. Please enable it in the Firebase Console.'
				);
			}

			throw error;
		}
	}

	/**
	 * Sign in with email and password
	 */
	async signInWithEmail(email: string, password: string): Promise<UserCredential> {
		if (!browser || !auth) {
			throw new Error(
				'Firebase authentication is not available. Please check your Firebase configuration.'
			);
		}

		try {
			const result = await signInWithEmailAndPassword(auth, email, password);
			return result;
		} catch (error: any) {
			console.error('Firebase email sign-in error:', error);

			// Provide specific guidance for common errors
			if (error.code === 'auth/user-not-found') {
				throw new Error('No account found with this email. Please sign up first.');
			} else if (error.code === 'auth/wrong-password') {
				throw new Error('Incorrect password. Please try again.');
			} else if (error.code === 'auth/invalid-credential') {
				throw new Error('Invalid email or password. Please check your credentials and try again.');
			} else if (error.code === 'auth/invalid-email') {
				throw new Error('Please enter a valid email address.');
			} else if (error.code === 'auth/user-disabled') {
				throw new Error('This account has been disabled. Please contact support.');
			} else if (error.code === 'auth/too-many-requests') {
				throw new Error('Too many failed attempts. Please try again later.');
			} else if (error.code === 'auth/operation-not-allowed') {
				throw new Error(
					'Email/password authentication is not enabled. Please enable it in the Firebase Console.'
				);
			}

			throw error;
		}
	}

	/**
	 * Send password reset email
	 */
	async sendPasswordReset(email: string): Promise<void> {
		if (!browser || !auth) {
			throw new Error(
				'Firebase authentication is not available. Please check your Firebase configuration.'
			);
		}

		try {
			await sendPasswordResetEmail(auth, email);
		} catch (error: any) {
			console.error('Firebase password reset error:', error);

			// Provide specific guidance for common errors
			if (error.code === 'auth/user-not-found') {
				throw new Error('No account found with this email address.');
			} else if (error.code === 'auth/invalid-email') {
				throw new Error('Please enter a valid email address.');
			}

			throw error;
		}
	}

	/**
	 * Resend email verification
	 */
	async resendEmailVerification(): Promise<void> {
		if (!browser || !auth || !auth.currentUser) {
			throw new Error('No user is currently signed in.');
		}

		try {
			await sendEmailVerification(auth.currentUser);
		} catch (error: any) {
			console.error('Firebase email verification error:', error);
			throw error;
		}
	}

	/**
	 * Sign out the current user
	 */
	async signOut(): Promise<void> {
		if (!browser || !auth) {
			return;
		}

		try {
			await signOut(auth);
		} catch (error) {
			console.error('Firebase sign-out error:', error);
			throw error;
		}
	}

	/**
	 * Get the current user's ID token
	 */
	async getCurrentUserIdToken(): Promise<string | null> {
		if (!browser || !auth || !auth.currentUser) {
			return null;
		}

		try {
			return await auth.currentUser.getIdToken();
		} catch (error) {
			console.error('Error getting ID token:', error);
			return null;
		}
	}

	/**
	 * Listen to auth state changes
	 */
	onAuthStateChanged(callback: (user: User | null) => void): Unsubscribe | null {
		if (!browser || !auth) {
			return null;
		}

		return onAuthStateChanged(auth, callback);
	}

	/**
	 * Get current user
	 */
	getCurrentUser(): User | null {
		if (!browser || !auth) {
			return null;
		}

		return auth.currentUser;
	}
}

export const firebaseAuth = FirebaseAuthService.getInstance();
