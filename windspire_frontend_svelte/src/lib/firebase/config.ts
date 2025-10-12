import { initializeApp } from 'firebase/app';
import { getAuth } from 'firebase/auth';
import { browser } from '$app/environment';

// Firebase configuration - these should be set via environment variables
const firebaseConfig = {
	apiKey: import.meta.env.VITE_FIREBASE_API_KEY,
	authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN,
	projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID,
	storageBucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET,
	messagingSenderId: import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID,
	appId: import.meta.env.VITE_FIREBASE_APP_ID
};

// Check if Firebase configuration is valid
function isFirebaseConfigValid(config: any): boolean {
	const hasAllFields =
		config.apiKey &&
		config.authDomain &&
		config.projectId &&
		config.storageBucket &&
		config.messagingSenderId &&
		config.appId;

	const hasValidValues =
		!config.apiKey.includes('your-firebase') &&
		!config.apiKey.includes('placeholder') &&
		!config.messagingSenderId.includes('123456789') &&
		!config.appId.includes('abcdef123456');

	return hasAllFields && hasValidValues;
}

// Initialize Firebase only in browser environment and with valid config
let app: any = null;
let auth: any = null;

if (browser && isFirebaseConfigValid(firebaseConfig)) {
	try {
		app = initializeApp(firebaseConfig);
		auth = getAuth(app);
		console.log('Firebase initialized successfully');
	} catch (error) {
		console.warn('Firebase initialization failed:', error);
		app = null;
		auth = null;
	}
} else if (browser) {
	console.warn(
		'Firebase configuration is invalid or contains placeholder values. Please set proper environment variables.'
	);
	console.warn('Current config validation status:', {
		hasApiKey: !!firebaseConfig.apiKey,
		hasAuthDomain: !!firebaseConfig.authDomain,
		hasProjectId: !!firebaseConfig.projectId,
		apiKeyValid:
			!firebaseConfig.apiKey?.includes('your-firebase') &&
			!firebaseConfig.apiKey?.includes('placeholder'),
		messagingSenderIdValid: !firebaseConfig.messagingSenderId?.includes('123456789'),
		appIdValid: !firebaseConfig.appId?.includes('abcdef123456')
	});
}

export { auth };
export default app;
