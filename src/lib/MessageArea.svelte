<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { Button, Textarea } from 'flowbite-svelte';
	import { AngleRightOutline, PaperClipOutline } from 'flowbite-svelte-icons';

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

	// Add event listener to the Textarea
	$: if (typeof window !== 'undefined') {
		document.addEventListener('keydown', handleKeydown);
	}
</script>

<div class="flex w-full max-w-full space-x-2 p-2" id="MessageBoxDiv">
	<Button outline size="xs"><PaperClipOutline /></Button>
	<Textarea
		bind:value={message}
		id="MessageBox"
		placeholder="Ask Lieutenant"
		rows="1"
		name="message"
		class="flex-grow p-3"
	/>
	<Button size="xs" id="SendMessageButton" on:click={handleSendMessage}
		><AngleRightOutline /></Button
	>
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
</style>
