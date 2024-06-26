<script>
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import imageUrl from '../lib/media/images/logo.png';
	import { Avatar, Textarea, Button, Fileupload } from 'flowbite-svelte';

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		console.log(output); // You can log the output or handle it as needed
	});
</script>

<div class="flex flex-col items-center space-y-4 p-4">
	<div class="p-2" id="AvatarDiv">
		<Avatar src={imageUrl} />
	</div>

	<div class="flex w-full max-w-screen-lg space-x-2 p-2" id="MessageBoxDiv">
		<Textarea
			id="MessageBox"
			placeholder="Ask Lieutenant"
			rows="1"
			name="message"
			class="flex-grow p-3"
		/>
		<Button color="blue" class="p-3">➤</Button>
	</div>
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
