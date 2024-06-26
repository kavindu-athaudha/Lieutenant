<script>
	import { onMount } from 'svelte';
	import Greet from '../lib/Greet.svelte';
	import { Command } from '@tauri-apps/api/shell';

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		console.log(output); // You can log the output or handle it as needed
	});
</script>

<h1>Welcome to SvelteKit</h1>
<Greet />