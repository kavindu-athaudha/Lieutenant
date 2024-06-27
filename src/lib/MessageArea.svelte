<script>
	import { Button, Textarea } from 'flowbite-svelte';
	import { AngleRightOutline, PaperClipOutline } from 'flowbite-svelte-icons';
</script>

<div class="flex w-full max-w-screen-lg space-x-2 p-2" id="MessageBoxDiv">
	<Button outline size="xs"><PaperClipOutline/></Button>
	<Textarea
		id="MessageBox"
		placeholder="Ask Lieutenant"
		rows="1"
		name="message"
		class="flex-grow p-3"
	/>
	<Button size="xs"><AngleRightOutline/></Button>
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
