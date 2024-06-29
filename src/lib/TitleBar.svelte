<script lang="ts">
	import { onMount } from "svelte";
	import { appWindow, LogicalSize } from "@tauri-apps/api/window";
	import { invoke } from "@tauri-apps/api/tauri";
	import { CloseOutline, MinusOutline } from "flowbite-svelte-icons";
  
	let minimizeButton: HTMLDivElement;
	let maximizeButton: HTMLDivElement;
	let closeButton: HTMLDivElement;
	let resize: HTMLDivElement;
  
	onMount(() => {
	  minimizeButton.addEventListener("click", () => invoke("minimize_to_tray"));
	  maximizeButton.addEventListener("click", () => appWindow.toggleMaximize());
	  closeButton.addEventListener("click", () => appWindow.close());
	  resize.addEventListener("click", () => appWindow.setSize(new LogicalSize(500, 1000)));
	});
  </script>
  
  <div data-tauri-drag-region class="titlebar">
	<div class="titlebar-button" bind:this={resize}>
	  <span class="material-symbols-outlined">fullscreen_portrait</span>
	</div>
	<div class="titlebar-button" bind:this={minimizeButton}>
	  <span class="material-symbols-outlined">remove</span>
	</div>
	<div class="titlebar-button" bind:this={maximizeButton}>
	  <span class="material-symbols-outlined">fullscreen</span>
	</div>
	<div class="titlebar-button" bind:this={closeButton}>
	  <span class="material-symbols-outlined">close</span>
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
	  color: var(--primary-color);
	}
  
	.titlebar-button:hover {
	  transform: scale(1.5);
	  opacity: 100%;
	}
  </style>
  