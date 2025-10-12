// Environment configuration for different deployment environments
export const config = {
	// API base URL - will be replaced by Vite at build time
	API_BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/api',

	// Firebase configuration - will be replaced by Vite at build time
	FIREBASE_CONFIG: import.meta.env.VITE_FIREBASE_CONFIG
		? JSON.parse(import.meta.env.VITE_FIREBASE_CONFIG)
		: {
				// Default local development config
				apiKey: 'demo-api-key',
				authDomain: 'windspire-demo.firebaseapp.com',
				projectId: 'windspire-demo',
				storageBucket: 'windspire-demo.appspot.com',
				messagingSenderId: '123456789',
				appId: '1:123456789:web:demo'
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
