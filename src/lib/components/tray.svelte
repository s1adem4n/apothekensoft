<script lang="ts">
	import { TrayIcon } from '@tauri-apps/api/tray';
	import { Menu } from '@tauri-apps/api/menu';
	import { exit } from '@tauri-apps/plugin-process';
	import { defaultWindowIcon } from '@tauri-apps/api/app';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	let loaded = $state(false);

	$effect(() => {
		if (loaded) return;
		loaded = true;

		load();
	});

	async function load() {
		const menu = await Menu.new({
			items: [
				{
					text: 'Öffnen',
					action: async () => {
						const window = getCurrentWindow();
						await window.show();
						await window.setFocus();
					}
				},
				{
					text: 'Beenden',
					action: () => {
						exit(0);
					}
				}
			]
		});

		const tray = await TrayIcon.new({
			menu,
			showMenuOnLeftClick: false,
			icon: (await defaultWindowIcon())!
		});
	}
</script>
