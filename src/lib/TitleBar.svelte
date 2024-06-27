<script lang="ts">
	import { onMount } from 'svelte';
	import { appWindow } from '@tauri-apps/api/window';

	let minimizeButton: HTMLDivElement;
	let maximizeButton: HTMLDivElement;
	let closeButton: HTMLDivElement;

	onMount(() => {
		minimizeButton.addEventListener('click', () => appWindow.minimize());
		maximizeButton.addEventListener('click', () => appWindow.toggleMaximize());
		closeButton.addEventListener('click', () => appWindow.close());
	});
</script>

<div data-tauri-drag-region class="titlebar">
	<div class="titlebar-button" bind:this={minimizeButton}>
		<img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
	</div>
	<div class="titlebar-button" bind:this={maximizeButton}>
		<img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
	</div>
	<div class="titlebar-button" bind:this={closeButton}>
		<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
	</div>
</div>

<style>
	.titlebar {
		height: 40pt;
		user-select: none;
		display: flex;
		justify-content: flex-end;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
	}

	.titlebar-button {
		display: inline-flex;
		justify-content: center;
		align-items: center;
		width: 30pt;
		height: 30pt;
		cursor: pointer;
		-webkit-app-region: no-drag; /* Prevents the buttons from being draggable */
		transition: transform 0.2s;
		opacity: 25%;
	}

	.titlebar-button:hover {
		transform: scale(1.5);
		opacity: 100%;
	}
</style>
