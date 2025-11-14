<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';

	import { Button } from '$lib/components/ui/button';

	$effect(() => {
		listen('global-key-event', (event) => {
			console.log('Received global key event:', event.payload);
		});

		invoke('simulate_key', { key: 'a', press: true })
			.then(() => {
				console.log('Simulated key press "a"');
			})
			.catch((err) => {
				console.error('Failed to simulate key press:', err);
			});
	});
</script>

<Button>Test</Button>
