<script>
	import { fly } from "svelte/transition";
	
	export let message = "";
	export let fromUser = true;
</script>

<div class="message-container {fromUser ? "from-user" : "received"}" transition:fly={{ x: fromUser ? 100 : -100, duration: 100 }}>
	<div class="chat-bubble">
		{message}
	</div>
</div>

<style>
	.message-container {
		display: flex;
		width: 100%;
		margin-bottom: 10px;
		font-family: "Noto Serif", serif;
		font-optical-sizing: auto;
		font-weight: 300;
		font-style: normal;
		font-variation-settings: "wdth" 100;
	}

	.from-user {
		justify-content: flex-end;
	}

	.received {
		justify-content: flex-start;
	}

	.chat-bubble {
		max-width: 60%;
		padding: 10pt;
		border-radius: 10pt;
		word-wrap: break-word;
	}

	.from-user .chat-bubble {
		background-color: var(--primary-color);
		color: #ffffff;
	}

	.received .chat-bubble {
		background-color: var(--secondary-color);
		color: #000000;
	}
</style>
