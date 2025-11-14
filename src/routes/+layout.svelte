<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<div class="flex h-full w-full flex-col items-center justify-center gap-4 p-4">
	{@render children?.()}
</div>
