// Svelte component declarations for TypeScript
declare module '*.svelte' {
    import type { Component } from 'svelte';
    const component: Component;
    export default component;
}

// Environment variables
declare global {
    namespace App {
        interface Locals { }
        interface PageData { }
        interface Platform { }
    }
}

export { };