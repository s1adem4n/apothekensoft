<script lang="ts">
	import { PUBLIC_POSTHOG_TOKEN } from '$env/static/public';
	import { getVersion } from '@tauri-apps/api/app';
	import { hostname } from '@tauri-apps/plugin-os';
	import posthog from 'posthog-js';

	let loaded = $state(false);

	$effect(() => {
		if (loaded) return;
		loaded = true;

		load();
	});

	async function load() {
		posthog.init(PUBLIC_POSTHOG_TOKEN, {
			api_host: 'https://eu.i.posthog.com',
			defaults: '2025-05-24',
			person_profiles: 'identified_only'
		});

		const version = await getVersion();
		posthog.register({
			app_version: version
		});

		const name = await hostname();
		if (name) {
			posthog.identify(name);
		}
	}
</script>
