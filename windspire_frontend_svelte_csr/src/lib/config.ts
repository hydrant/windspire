// Environment configuration for different deployment environments
export const config = {
	// API base URL - will be replaced by Vite at build time
	API_BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/api',

	// Firebase configuration - will be replaced by Vite at build time
	FIREBASE_CONFIG: {
		apiKey: import.meta.env.VITE_FIREBASE_API_KEY || 'demo-api-key',
		authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN || 'windspire-demo.firebaseapp.com',
		projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID || 'windspire-demo',
		storageBucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET || 'windspire-demo.appspot.com',
		messagingSenderId: import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID || '123456789',
		appId: import.meta.env.VITE_FIREBASE_APP_ID || '1:123456789:web:demo'
	},

	// Environment detection
	isDevelopment: import.meta.env.DEV,
	isProduction: import.meta.env.PROD,

	// App metadata
	APP_NAME: 'Windspire',
	APP_VERSION: '1.0.0'
};

// Type definitions for global environment variables
declare global {
	const __API_BASE_URL__: string;
	const __FIREBASE_CONFIG__: string;
}
