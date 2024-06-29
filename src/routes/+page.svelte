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

	function addUserMessage(event: MessageEvent){
		messages = [...messages, { role: "user", content: event.detail.text }];
		getReplyFromLLM(messages);
	}

	function addAssistantMessage(message: string){
		messages = [...messages, { role: "assistant", content: message }];
	}

	async function getReplyFromLLM(messages: Message[]){
		const userMessages = messages.map(msg => ({ role: msg.role, content: msg.content }));

		try {
			const response = await fetch('https://api.openai.com/v1/chat/completions', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'Authorization': `Bearer $YOUR_API_KEY`
				},
				body: JSON.stringify({
					model: "gpt-4", // Use the correct model name
					messages: userMessages,
					max_tokens: 150,
					n: 1,
					stop: null,
					temperature: 0.7
				})
			});
			
			if (!response.ok) {
				const errorData = await response.json();
				console.error("Error from OpenAI API:", errorData);
				return;
			}

			const data = await response.json();
			const replyFromLLM = data.choices[0].message.content;
			addAssistantMessage(replyFromLLM);
		} catch (error) {
			console.error("Network error:", error);
		}
	}

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		isLoading = false;

		setTimeout(() => {
			addAssistantMessage("Hello! How can I help you today?");
		}, 500);
	});
</script>

{#if isLoading}
	<SplashScreen />
{:else}
	<div class="flex flex-col items-center space-y-4 p-4">
		<Header />
		<MessageArea on:messageSent={addUserMessage} />
		{#each messages as { content, role }}
			<ChatBubble content={content} {role} />
		{/each}
	</div>
{/if}
