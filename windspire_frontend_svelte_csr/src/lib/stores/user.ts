import { writable } from 'svelte/store';

export interface User {
    id: string;
    email: string;
    name: string;
    picture?: string;
}

// User store for authentication state
export const userStore = writable<User | null>(null);