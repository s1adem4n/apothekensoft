<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';

	import { Button } from '$lib/components/ui/button';
	import { parseGS1, type GS1Data } from '$lib/parser';

	let string = $state('');
	let data: GS1Data | null = $state(null);

	$effect(() => {
		listen<{ label: string | null }>('global-key-event', (event) => {
			if (event.payload.label) {
				string += event.payload.label;
			}

			const index = string.indexOf('0104150');
			if (index !== -1) {
				setTimeout(() => {
					data = parseGS1(string.substring(index));
				}, 250);
			}
		});

		setInterval(() => {
			invoke('simulate_key', { key: 'a', press: true }).catch((err) => {
				alert(`Failed to simulate key press: ${err}`);
			});
		}, 1000);
	});
</script>

<pre>{JSON.stringify(data, null, 2)}</pre>
