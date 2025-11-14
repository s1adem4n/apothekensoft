<script lang="ts">
	import { listen } from '@tauri-apps/api/event';

	let string = $state('');
	let events = $state<any>([]);

	$effect(() => {
		listen<{ label: string | null }>('global-key-event', (event) => {
			events.push(event.payload);
		});
	});
</script>

<input bind:value={string} />
<pre>{string}</pre>
<pre class="overflow-y-auto">{JSON.stringify(events, null, 2)}</pre>
