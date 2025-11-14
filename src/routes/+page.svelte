<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';

	import { Button } from '$lib/components/ui/button';

	let string = $state('');

	$effect(() => {
		listen<{ label: string | null }>('global-key-event', (event) => {
			if (event.payload.label) {
				string += event.payload.label;
			}
		});
	});
</script>

<p>{string}</p>

<Button
	onclick={() => {
		invoke('simulate_key', { key: 'a', press: true })
			.then(() => {
				alert('Simulated key press of "a"');
			})
			.catch((err) => {
				alert(`Failed to simulate key press: ${err}`);
			});
	}}
>
	Simulate a key
</Button>
