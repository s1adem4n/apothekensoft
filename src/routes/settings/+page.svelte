<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { store } from '$lib/store.svelte';
	import { check } from '@tauri-apps/plugin-updater';
	import IconArrowLeft from '~icons/mdi/arrow-left';

	let updateAvailable = $state(false);
	$effect(() => {
		check().then((update) => {
			updateAvailable = update !== null;
		});
	});
</script>

<Button
	variant="ghost"
	size="icon"
	href="/"
	class="absolute top-2 left-2"
	style="view-transition-name: header-button;"
>
	<IconArrowLeft />
</Button>

<div class="flex w-full flex-1 flex-col gap-4 p-4">
	<h1 class="text-center font-semibold">Einstellungen</h1>

	<div class="flex flex-1 flex-col gap-4 overflow-y-auto">
		<div class="flex items-center justify-between">
			<span>Verfalldatums-Erkennung</span>
			<Switch bind:checked={store.settings.expiryDetection.enabled} />
		</div>
		<div class="flex items-center justify-between">
			<span>Eingabe vorher bestätigen</span>
			<Switch bind:checked={store.settings.expiryDetection.confirmation} />
		</div>

		{#if updateAvailable}
			<Button variant="link" href="/update">Ein Update ist verfügbar!</Button>
		{/if}
	</div>
</div>
