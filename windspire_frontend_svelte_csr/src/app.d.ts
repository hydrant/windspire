// Svelte component declarations for TypeScript
declare module '*.svelte' {
	import type { Component } from 'svelte';
	const component: Component;
	export default component;
}

// Environment variables
declare global {
	namespace App {
		// eslint-disable-next-line @typescript-eslint/no-empty-object-type
		interface Locals { }
		// eslint-disable-next-line @typescript-eslint/no-empty-object-type
		interface PageData { }
		// eslint-disable-next-line @typescript-eslint/no-empty-object-type
		interface Platform { }
	}
}

export { };
