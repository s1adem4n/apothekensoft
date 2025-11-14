import { load, Store as TauriStore } from '@tauri-apps/plugin-store';

export interface Settings {
	expiryDetection: {
		enabled: boolean;
		confirmation: boolean;
	};
}

export class Store {
	private store: TauriStore | null = null;
	private initialized = false;

	settings: Settings = $state({
		expiryDetection: {
			enabled: true,
			confirmation: false
		}
	});

	async init() {
		if (this.initialized) return;
		this.initialized = true;

		this.store = await load('store.json');

		$effect.root(() => {
			$effect(() => {
				if (!this.store) return;

				this.save();
			});
		});
	}

	async load() {
		if (!this.store) return;

		const settings = await this.store.get<Settings>('settings');
		if (settings) {
			this.settings = settings;
		}
	}

	async save() {
		if (!this.store) return;

		await this.store.set('settings', this.settings);
		await this.store.save();
	}
}

export const store = new Store();
