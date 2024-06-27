<script>
	export let message = '';
	export let fromUser = true; // true for messages sent by the user, false for received messages
</script>

<div class="message-container {fromUser ? 'from-user' : 'received'}">
	<div class="chat-bubble">
		{message}
	</div>
</div>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Noto+Serif:ital,wdth,wght@0,62.5..100,100..900;1,62.5..100,100..900&display=swap');

	.message-container {
		display: flex;
		width: 100%;
		margin-bottom: 10px;
		font-family: 'Noto Serif', serif;
		font-optical-sizing: auto;
		font-weight: 300;
		font-style: normal;
		font-variation-settings: 'wdth' 100;
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
		color: #ffffff; /* Added color for better readability */
	}

	.received .chat-bubble {
		background-color: var(--secondary-color);
		color: #000000; /* Added color for better readability */
	}
</style>
