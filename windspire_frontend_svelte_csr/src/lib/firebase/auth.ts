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
		if (!auth) {
			console.warn('Firebase auth not available - Firebase not properly initialized');
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
		} catch (error: unknown) {
			console.error('Firebase Google sign-in error:', error);

			// Provide specific guidance for common errors
			if (error && typeof error === 'object' && 'code' in error) {
				const firebaseError = error as { code: string; message: string };
				if (firebaseError.code === 'auth/operation-not-allowed') {
					throw new Error(
						'Google Sign-In is not enabled in Firebase. Please enable it in the Firebase Console under Authentication > Sign-in method > Google.'
					);
				} else if (firebaseError.code === 'auth/unauthorized-domain') {
					throw new Error(
						'This domain is not authorized for OAuth operations. Please add it to the Firebase Console under Authentication > Settings > Authorized domains.'
					);
				} else if (firebaseError.code === 'auth/popup-blocked') {
					throw new Error(
						'Sign-in popup was blocked by the browser. Please allow popups for this site and try again.'
					);
				} else if (firebaseError.code === 'auth/popup-closed-by-user') {
					throw new Error('Sign-in popup was closed before completing. Please try again.');
				} else if (firebaseError.code === 'auth/cancelled-popup-request') {
					// User cancelled - this is normal, just return null
					return null;
				}
			}

			throw new Error(
				`Sign-in failed: ${error instanceof Error ? (error instanceof Error ? error.message : 'Unknown error') : 'Unknown error'}`
			);
		}
	}

	/**
	 * Sign in with email and password
	 */
	async signInWithEmailAndPassword(email: string, password: string): Promise<UserCredential> {
		if (!auth) {
			throw new Error('Firebase authentication is not available.');
		}

		try {
			return await signInWithEmailAndPassword(auth, email, password);
		} catch (error: unknown) {
			console.error('Email sign-in error:', error);
			throw new Error(
				`Sign-in failed: ${error instanceof Error ? (error instanceof Error ? error.message : 'Unknown error') : 'Unknown error'}`
			);
		}
	}

	/**
	 * Create user with email and password
	 */
	async createUserWithEmailAndPassword(email: string, password: string): Promise<UserCredential> {
		if (!auth) {
			throw new Error('Firebase authentication is not available.');
		}

		try {
			return await createUserWithEmailAndPassword(auth, email, password);
		} catch (error: unknown) {
			console.error('User creation error:', error);
			throw new Error(
				`Account creation failed: ${error instanceof Error ? (error instanceof Error ? error.message : 'Unknown error') : 'Unknown error'}`
			);
		}
	}

	/**
	 * Sign up with email, password, and display name
	 */
	async signUpWithEmailAndPassword(
		email: string,
		password: string,
		displayName: string
	): Promise<UserCredential> {
		if (!auth) {
			throw new Error('Firebase authentication is not available.');
		}

		try {
			// Create user account
			const userCredential = await createUserWithEmailAndPassword(auth, email, password);

			// Update user profile with display name
			if (displayName && userCredential.user) {
				await updateProfile(userCredential.user, { displayName });
			}

			return userCredential;
		} catch (error: unknown) {
			console.error('User creation error:', error);
			throw new Error(
				`Account creation failed: ${error instanceof Error ? (error instanceof Error ? error.message : 'Unknown error') : 'Unknown error'}`
			);
		}
	}

	/**
	 * Send password reset email
	 */
	async sendPasswordResetEmail(email: string): Promise<void> {
		if (!auth) {
			throw new Error('Firebase authentication is not available.');
		}

		try {
			await sendPasswordResetEmail(auth, email);
		} catch (error: unknown) {
			console.error('Password reset error:', error);
			throw new Error(
				`Password reset failed: ${error instanceof Error ? (error instanceof Error ? error.message : 'Unknown error') : 'Unknown error'}`
			);
		}
	}

	/**
	 * Send email verification
	 */
	async sendEmailVerification(user: User): Promise<void> {
		try {
			await sendEmailVerification(user);
		} catch (error: unknown) {
			console.error('Email verification error:', error);
			throw new Error(
				`Email verification failed: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Update user profile
	 */
	async updateUserProfile(
		user: User,
		profile: { displayName?: string; photoURL?: string }
	): Promise<void> {
		try {
			await updateProfile(user, profile);
		} catch (error: unknown) {
			console.error('Profile update error:', error);
			throw new Error(
				`Profile update failed: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Sign out current user
	 */
	async signOut(): Promise<void> {
		if (!auth) {
			throw new Error('Firebase authentication is not available.');
		}

		try {
			await signOut(auth);
		} catch (error: unknown) {
			console.error('Sign-out error:', error);
			throw new Error(
				`Sign-out failed: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}

	/**
	 * Get current user
	 */
	getCurrentUser(): User | null {
		return auth?.currentUser || null;
	}

	/**
	 * Listen to auth state changes
	 */
	onAuthStateChanged(callback: (user: User | null) => void): Unsubscribe {
		if (!auth) {
			throw new Error('Firebase authentication is not available.');
		}

		return onAuthStateChanged(auth, callback);
	}

	/**
	 * Check if user is signed in
	 */
	isSignedIn(): boolean {
		return !!auth?.currentUser;
	}

	/**
	 * Get ID token for current user
	 */
	async getIdToken(forceRefresh: boolean = false): Promise<string | null> {
		const user = this.getCurrentUser();
		if (!user) {
			return null;
		}

		try {
			return await user.getIdToken(forceRefresh);
		} catch (error: unknown) {
			console.error('Token retrieval error:', error);
			throw new Error(
				`Token retrieval failed: ${error instanceof Error ? error.message : 'Unknown error'}`
			);
		}
	}
}

// Export singleton instance
export const firebaseAuthService = FirebaseAuthService.getInstance();
