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
	.message-container {
		display: flex;
		width: 100%;
		margin-bottom: 10px;
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
		background-color: #dcf8c6;
	}

	.received .chat-bubble {
		background-color: #dcf8c6;
	}
</style>
