<script lang="ts">
	import IconLoading from '~icons/mdi/loading';
	import { cn } from '$lib/utils.js';
	import type { ComponentProps } from 'svelte';

	let { class: className, ...restProps }: ComponentProps<typeof IconLoading> = $props();
</script>

<IconLoading
	role="status"
	aria-label="Loading"
	class={cn('size-4 animate-spin', className)}
	{...restProps}
/>
