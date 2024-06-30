<script lang="ts">
	import { Textarea, Button } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';

	let message = '';
	const dispatch = createEventDispatcher();

	function handleSendMessage() {
		if (message.trim()) {
			dispatch('messageSent', { text: message });
			message = '';
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			handleSendMessage();
		}
	}

	$: if (typeof window !== 'undefined') {
		document.addEventListener('keydown', handleKeydown);
	}
</script>

<div class="flex items-center rounded-lg bg-gray-50 px-3 py-2 dark:bg-gray-700" id="MessageBoxDiv">
	<button type="button" class="chat-button" aria-label="Attach file">
		<span class="material-symbols-outlined" style="padding-right: 0.25em">attach_file</span>
	</button>
	<Textarea
		bind:value={message}
		id="chat"
		class="mx-0 bg-white"
		rows="1"
		placeholder="Ask Lieutenant..."
	/>
	<button type="button" class="chat-button" aria-label="Send message" on:click={handleSendMessage}>
		<span class="material-symbols-outlined" style="padding-left: 0.25em">send</span>
	</button>
</div>

<style>
	#MessageBoxDiv {
		position: fixed;
		bottom: 0;
		width: 100%;
		background-color: white;
		padding-bottom: 15pt;
		padding-left: 15pt;
		padding-right: 15pt;
	}

	.chat-button {
		color: var(--primary-color);
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform 0.2s;
	}

	.chat-button:hover {
		transform: scale(1.5);
		opacity: 100%;
	}
</style>
