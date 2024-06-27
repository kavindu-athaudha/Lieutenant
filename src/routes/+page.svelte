<script>
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import Header from '$lib/Header.svelte';
	import MessageArea from '$lib/MessageArea.svelte';
	import ChatBubble from '$lib/ChatBubble.svelte';

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		console.log(output); // You can log the output or handle it as needed
	});

	let messages = [
		{ text: 'Hello! How can I help you today?', fromUser: false },
		{ text: 'I need some information about your services.', fromUser: true }
	];
</script>

<div class="flex flex-col items-center space-y-4 p-4">
	<Header />
	<MessageArea />
	{#each messages as { text, fromUser }}
		<ChatBubble message={text} {fromUser} />
	{/each}
</div>
