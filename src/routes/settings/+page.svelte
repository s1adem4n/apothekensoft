<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	import { store } from '$lib/store.svelte';
	import IconArrowLeft from '~icons/mdi/arrow-left';
</script>

<div class="flex flex-col gap-4 p-4">
	<Button variant="ghost" size="icon" href="/" class="absolute top-2 left-2">
		<IconArrowLeft />
	</Button>

	<h1 class="text-center font-semibold">Einstellungen</h1>

	<div class="flex flex-1 flex-col gap-4 overflow-y-auto">
		<div class="flex items-center justify-between">
			<span>Verfalldatums-Erkennung</span>
			<Switch bind:checked={store.settings.enabled} />
		</div>
		<div class="flex items-center justify-between">
			<span>Eingabe vorher bestätigen</span>
			<Switch bind:checked={store.settings.askForConfirmation} />
		</div>
	</div>
</div>
