<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { check, type Update } from '@tauri-apps/plugin-updater';
	import { relaunch } from '@tauri-apps/plugin-process';
	import IconArrowBack from '~icons/mdi/arrow-left';

	let loading = $state(true);
	let update: Update | null = $state(null);

	$effect(() => {
		loading = true;
		check().then((result) => {
			update = result;
			loading = false;
		});
	});
</script>

{#if !loading}
	<Button variant="ghost" size="icon" href="/settings" class="absolute top-2 left-2">
		<IconArrowBack />
	</Button>
{/if}

{#if loading}
	<Spinner />
{:else if update}
	<p>Ein Update ist verfügbar!</p>
	<Button
		onclick={async () => {
			loading = true;
			await update?.downloadAndInstall();
			await relaunch();
		}}
	>
		Jetzt aktualisieren
	</Button>
{:else}
	<p>Die App ist auf dem neuesten Stand!</p>
{/if}
