<script>
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import Header from '$lib/Header.svelte';
	import MessageArea from '$lib/MessageArea.svelte';

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		console.log(output); // You can log the output or handle it as needed
	});
</script>

<div class="flex flex-col items-center space-y-4 p-4">
	<Header />
	<MessageArea />
</div>