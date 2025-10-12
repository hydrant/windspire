import type { Config } from 'tailwindcss';

const config: Config = {
	content: ['./index.html', './src/**/*.{js,ts,svelte}'],
	theme: {
		extend: {}
	},
	plugins: []
};

export default config;
