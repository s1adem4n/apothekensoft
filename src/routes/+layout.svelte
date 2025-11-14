<script lang="ts">
	import '../app.css';
	import { store } from '$lib/store.svelte';
	import { Spinner } from '$lib/components/ui/spinner';

	let { children } = $props();

	let loaded = $state(false);

	$effect(() => {
		if (loaded) return;
		load();
	});

	async function load() {
		await store.init();
		loaded = true;
	}
</script>

<div class="flex h-full w-full flex-col gap-4">
	{#if loaded}
		{@render children?.()}
	{:else}
		<Spinner />
	{/if}
</div>
