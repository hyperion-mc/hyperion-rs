<script lang="ts">
	import '../app.css';

	const { children } = $props();


</script>

<main>
	{@render children?.()}
</main>

<style>
    main {
        margin: 0;
        padding: 40px 20px;
        display: flex;
        flex-direction: column;
        align-items: center;
        height: 100vh;
    }
</style>
