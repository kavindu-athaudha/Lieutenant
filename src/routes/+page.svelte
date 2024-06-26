<script>
	import { onMount } from 'svelte';
	import { Command } from '@tauri-apps/api/shell';
	import { Alert } from 'flowbite-svelte';

	onMount(async () => {
		const command = Command.sidecar('binaries/backend');
		const output = await command.execute();
		console.log(output); // You can log the output or handle it as needed
	});
</script>

<div class="p-8">
    <Alert>
      <span class="font-medium">Info alert!</span>
      Change a few things up and try submitting again.
    </Alert>
  </div>