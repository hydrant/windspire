import { initializeApp } from 'firebase/app';
import { getAuth } from 'firebase/auth';
import { config } from '../config';

// Initialize Firebase with configuration
const app = initializeApp(config.FIREBASE_CONFIG);

// Initialize Firebase Auth
export const auth = getAuth(app);

// Set the app language to the user's preferred language
auth.languageCode = 'en';

export default app;