<script>
	import logoURL from '$lib/media/images/logo.svg';
	import { Alert } from 'flowbite-svelte';
	import { InfoCircleSolid } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	let isWarningVisible = false;

	onMount(() => {
		setTimeout(() => {
			isWarningVisible = true;
		}, 1000);
	});
</script>

<div class="splash-screen">
	<div class="logo-container">
		<img src={logoURL} alt="Logo" class="logo" />
	</div>

	{#if isWarningVisible}
		<div id="StartUpWarning" class="alert-container">
			<Alert border color="yellow">
				<InfoCircleSolid slot="icon" class="h-5 w-5" />
				This is taking longer than usual, try restarting the Lieutenant.
			</Alert>
		</div>
	{/if}
</div>

<style>
	.splash-screen {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100vh;
		position: relative;
	}
	.logo {
		width: 10vw;
	}
	.logo-container,
	.alert-container {
		margin-bottom: 20px;
	}

	#StartUpWarning {
		position: absolute;
		bottom: 30%;
		width: 80vw;
		padding-top: 30pt;
	}
</style>
