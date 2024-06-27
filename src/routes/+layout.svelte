<script>
    import '../app.css';
    import TitleBar from '$lib/TitleBar.svelte'
</script>

<div class="layout">
    <TitleBar/>
	<slot></slot>
</div>
