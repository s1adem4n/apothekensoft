<script lang="ts">
	import IconCog from '~icons/mdi/cog';
	import { listen } from '@tauri-apps/api/event';
	import Button from '$lib/components/ui/button/button.svelte';
	import { parseGS1, type GS1Data, type KeyEvent } from '$lib/gs1';
	import { confirm } from '@tauri-apps/plugin-dialog';
	import { store } from '$lib/store.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let buffer: KeyEvent[] = $state([]);
	let scanning = $state(false);
	let data: GS1Data | null = $state(null);
	let lastInput = $state(new Date());

	$effect(() => {
		if (!data?.expirationDate) return;
		sendExpiration();
	});

	async function sendExpiration() {
		if (!data?.expirationDate) return;
		if (!store.settings.expiryDetection.enabled) return;

		if (store.settings.expiryDetection.confirmation) {
			const month = data.expirationDate.substring(2, 4);
			const year = data.expirationDate.substring(0, 2);
			const result = await confirm(
				`Ablaufdatum ${month}/${year} eingeben?`,
				'Ablaufdatum eingeben'
			);
			if (!result) return;
		}

		const keys = data.expirationDate.substring(2);
		for (const key of keys) {
			await invoke('simulate_key', { key });
		}
	}

	$effect(() => {
		setInterval(() => {
			const now = new Date();
			if (scanning && now.getTime() - lastInput.getTime() > 500) {
				// Timeout after 500ms of inactivity
				scanning = false;
				buffer = [];
			}
		}, 500);

		listen<{ key_code: string; label: string | null; state: string }>(
			'global-key-event',
			(event) => {
				lastInput = new Date();

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

<Button
	variant="ghost"
	size="icon"
	href="/settings"
	class="absolute top-2 right-2"
	style="view-transition-name: header-button;"
>
	<IconCog />
</Button>

<div class="flex h-full w-full flex-col gap-4 p-4">
	<h1 class="text-center font-semibold">Scanner</h1>

	<div class="flex flex-1 flex-col gap-4 overflow-y-auto">
		{#if scanning}
			<div>Scannt...</div>
		{:else}
			<div>Warte auf Scan...</div>
		{/if}

		{#if data}
			<div class="grid grid-cols-2">
				<span>PZN</span>
				<span>{data?.gtin?.substring(5)}</span>

				<span>Verfallsdatum</span>
				<span>{data?.expirationDate}</span>

				<span>Charge</span>
				<span>{data?.lotNumber}</span>

				<span>Seriennummer</span>
				<span>{data?.serialNumber}</span>
			</div>
		{/if}
	</div>
</div>
