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
			const response = await fetch('https://api.openai.com/v1/chat/completions', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'Authorization': `Bearer $API_TOKEN`
				},
				body: JSON.stringify({
					model: 'gpt-4o', // Use the correct model name
					messages: userMessages,
					max_tokens: 150,
					n: 1,
					stop: null,
					temperature: 0.7,
					stream: true // Enable streaming
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				console.error('Error from OpenAI API:', errorData);
				return;
			}

			const reader = response.body?.getReader();
			const decoder = new TextDecoder('utf-8');
			let fullResponse = '';

			while (true) {
				const { done, value } = (await reader?.read()) || {};
				if (done) break;
				const chunk = decoder.decode(value, { stream: true });

				// Log the full chunk
				console.log('Full chunk:', chunk);

				// Extract and log the content of each chunk
				const dataMatches = chunk.matchAll(/"content":"([^"]*)"/g);
				for (const match of dataMatches) {
					if (match) {
						let partialResponse = match[1];
						// Replace special characters if needed
						partialResponse = partialResponse.replace(/\\n/g, '\n').replace(/\\"/g, '"');
						console.log('Partial Response:', partialResponse);
						fullResponse += partialResponse;
					}
				}

				// End loop if "data: [DONE]" is received
				if (chunk.includes('data: [DONE]')) {
					console.log('Received "DONE". Ending loop.');
					break;
				}
			}

			addAssistantMessage(fullResponse);
			console.log('Full response: ' + fullResponse);
		} catch (error) {
			console.error('Network error:', error);
		}
	}

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		isLoading = false;
		addAssistantMessage('Hello! How can I help you today?');
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
