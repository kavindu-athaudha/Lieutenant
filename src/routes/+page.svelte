<script lang="ts">
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import Header from '$lib/Header.svelte';
	import MessageArea from '$lib/MessageArea.svelte';
	import ChatBubble from '$lib/ChatBubble.svelte';
	import SplashScreen from '$lib/SplashScreen.svelte';

	let isLoading = true;

	type Message = {
		role: string;
		content: string;
	};

	type MessageEvent = CustomEvent<{ text: string }>;

	let messages: Message[] = [];

	function addUserMessage(event: MessageEvent) {
		messages = [...messages, { role: 'user', content: event.detail.text }];
		getReplyFromLLM(messages);
	}

	function addAssistantMessage(message: string) {
		messages = [...messages, { role: 'assistant', content: message }];
	}

	async function getReplyFromLLM(messages: Message[]) {
		const userMessages = messages.map((msg) => ({ role: msg.role, content: msg.content }));

		try {
			const response = await fetch('http://localhost:1999/ask', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(userMessages)
			});
			console.log(JSON.stringify(userMessages));
			debugger;

			if (!response.ok) {
				const errorData = await response.json();
				console.error('Error from FastAPI:', errorData);
				return;
			}

			const data = await response.json();
			addAssistantMessage(data.reply);
		} catch (error) {
			console.error('Network error:', error);
		}
	}

	async function initializeBackend() {
		const command = Command.sidecar('binaries/backend');
		await command.execute();
		addAssistantMessage('Hello! How can I help you today?');
	}

	onMount(async () => {
		isLoading = false;
		initializeBackend();
	});
</script>

{#if isLoading}
	<SplashScreen />
{:else}
	<div class="flex flex-col items-center space-y-4 p-4">
		<Header />
		<MessageArea on:messageSent={addUserMessage} />
		<div class="messages-container">
			{#each messages as { content, role }}
				<ChatBubble {content} {role} />
			{/each}
		</div>
	</div>
{/if}

<style>
	.messages-container {
		flex: 1;
		width: 100%;
		overflow-y: auto;
		padding: 1rem;
		border-radius: 10px;
		max-height: 85vh; /* Adjust this value based on your layout */
	}
</style>
