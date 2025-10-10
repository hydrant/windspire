import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
    plugins: [svelte()],
    css: {
        postcss: './postcss.config.js'
    },
    build: {
        outDir: 'dist',
        assetsDir: 'assets',
        sourcemap: true,
        rollupOptions: {
            output: {
                manualChunks: {
                    vendor: ['svelte'],
                    firebase: ['firebase/app', 'firebase/auth'],
                    router: ['@mateothegreat/svelte5-router']
                }
            }
        }
    },
    server: {
        port: 3000,
        host: true
    },
    preview: {
        port: 3000,
        host: true
    },
    resolve: {
        alias: {
            '$lib': '/src/lib'
        }
    }
});