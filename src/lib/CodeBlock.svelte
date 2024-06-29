<script lang="ts">
	import { Highlight } from 'svelte-highlight';
	import javascript from 'svelte-highlight/languages/javascript';
	import python from 'svelte-highlight/languages/python';
	import 'svelte-highlight/styles/github.css';

	export let code: string = `const greet = () => { console.log('Hello, world!'); };`;
	export let language: string = 'javascript';

	// Define a type for the languages object
	type LanguageMap = {
		[key: string]: any;
	};

	// Mapping language names to their respective imports
	const languages: LanguageMap = {
		javascript,
		python
		// Add more languages as needed
	};
</script>

<Highlight language={languages[language]} {code} />
