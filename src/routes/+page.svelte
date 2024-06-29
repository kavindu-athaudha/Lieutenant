<script lang="ts">
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import Header from '$lib/Header.svelte';
	import MessageArea from '$lib/MessageArea.svelte';
	import ChatBubble from '$lib/ChatBubble.svelte';
	import SplashScreen from '$lib/SplashScreen.svelte';

	let isLoading = true;

	type Message = {
		text: string;
		fromUser: boolean;
	};

	type MessageEvent = CustomEvent<{ text: string }>;

	let messages: Message[] = [];

	function addMessage(event: MessageEvent) {
		messages = [...messages, { text: event.detail.text, fromUser: true }];
	}

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		isLoading = false;

		setTimeout(() => {
			messages = [...messages, { text: 'Hello! How can I help you today?', fromUser: false }];
		}, 500);
	});
</script>

{#if isLoading}
	<SplashScreen />
{:else}
	<div class="flex flex-col items-center space-y-4 p-4">
		<Header />
		<MessageArea on:messageSent={addMessage} />
		{#each messages as { text, fromUser }}
			<ChatBubble message={text} {fromUser} />
		{/each}
	</div>
{/if}
