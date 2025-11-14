<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';

	import { Button } from '$lib/components/ui/button';

	let events: any[] = $state([]);

	$effect(() => {
		listen<{ key_code: string; label: string | null; state: string }>(
			'global-key-event',
			(event) => {
				events.push(event);
			}
		);
	});
</script>

<pre class="w-full overflow-y-auto">
  {JSON.stringify(events, null, 2)}
</pre>

<Button
	onclick={() => {
		invoke('simulate_key', { key: 'a', press: true })
			.then(() => {
				console.log('Simulated key press "a"');
			})
			.catch((err) => {
				console.error('Failed to simulate key press:', err);
			});
	}}
>
	Test
</Button>
