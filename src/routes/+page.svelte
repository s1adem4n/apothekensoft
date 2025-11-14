<script lang="ts">
	import IconCog from '~icons/mdi/cog';
	import { listen } from '@tauri-apps/api/event';
	import Button from '$lib/components/ui/button/button.svelte';
	import { parseGS1, type GS1Data, type KeyEvent } from '$lib/gs1';

	let scannedData = $state('');
	let buffer: KeyEvent[] = $state([]);
	let scanning = $state(false);
	let data: GS1Data | null = $state(null);

	$effect(() => {
		listen<{ key_code: string; label: string | null; state: string }>(
			'global-key-event',
			(event) => {
				const { label, state } = event.payload;
				if (state !== 'down') return;

				// Check for start of scan (STX)
				if (!scanning && label === '\u0002') {
					scanning = true;
					buffer = [];
					return;
				}

				// Check for end of scan (ETX)
				if (scanning && label === '\u0003') {
					scanning = false;
					data = parseGS1(buffer);
					buffer = [];
					return;
				}

				// If scanning, add to buffer
				if (scanning) {
					buffer.push(event.payload);
				}
			}
		);
	});
</script>

<div class="flex flex-col gap-4 p-4">
	<Button variant="ghost" size="icon" href="/settings" class="absolute top-2 right-2">
		<IconCog />
	</Button>

	<div class="flex flex-1 flex-col gap-4 overflow-y-auto">
		{#if data}
			<pre>{JSON.stringify(data, null, 2)}</pre>
		{/if}
		{#if scanning}
			<div>Scannt...</div>
		{:else}
			<div>Warte auf Scan...</div>
		{/if}
	</div>
</div>
