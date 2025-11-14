<script lang="ts">
	import { TrayIcon } from '@tauri-apps/api/tray';
	import { Menu } from '@tauri-apps/api/menu';
	import { exit } from '@tauri-apps/plugin-process';
	import { defaultWindowIcon } from '@tauri-apps/api/app';
	import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

	let loaded = $state(false);

	$effect(() => {
		if (loaded) return;
		loaded = true;

		load();
	});

	async function showWindow() {
		const window = getCurrentWindow();
		await window.show();
		await window.setFocus();
	}

	async function load() {
		const menu = await Menu.new({
			items: [
				{
					text: 'Öffnen',
					action: async () => {
						await showWindow();
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

		await TrayIcon.new({
			menu,
			showMenuOnLeftClick: false,
			action: (e) => {
				if (e.type === 'Click' && e.button === 'Left') {
					showWindow();
				}
			},
			icon: (await defaultWindowIcon())!
		});
	}
</script>
