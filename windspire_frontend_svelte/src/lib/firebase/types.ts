export interface FirebaseAuthRequest {
    id_token: string; // Changed from idToken to match backend snake_case
}

export interface FirebaseAuthResponse {
    success: boolean;
    data?: {
        token: string; // Our backend JWT token
        user: {
            id: string;
            email: string;
            name: string;
            picture?: string;
        };
    };
    message?: string;
}

export interface FirebaseUser {
    uid: string;
    email: string | null;
    displayName: string | null;
    photoURL: string | null;
    emailVerified: boolean;
}