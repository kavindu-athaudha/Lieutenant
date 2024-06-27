import flowbitePlugin from 'flowbite/plugin';

import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			colors: {
				// flowbite-svelte
				primary: {
					50: '#1e3a8a',
					100: '#1e3a8a',
					200: '#1e3a8a',
					300: '#1e3a8a',
					400: '#1e3a8a',
					500: '#1e3a8a',
					600: '#1e3a8a',
					700: '#1e3a8a',
					800: '#1e3a8a',
					900: '#1e3a8a'
				}
			}
		}
	},

	plugins: [flowbitePlugin]
} as Config;
