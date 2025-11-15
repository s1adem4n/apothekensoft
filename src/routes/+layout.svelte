<script lang="ts">
	import '../app.css';
	import { store } from '$lib/store.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import { onNavigate } from '$app/navigation';

	import Tray from '$lib/components/tray.svelte';
	import Posthog from '$lib/components/posthog.svelte';

	onNavigate((navigation) => {
		if (!document.startViewTransition) return;

		return new Promise((resolve) => {
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		});
	});

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

<Tray />
<Posthog />

<div class="flex w-full flex-1 flex-col items-center justify-center gap-4">
	{#if loaded}
		{@render children?.()}
	{:else}
		<Spinner />
	{/if}
</div>
