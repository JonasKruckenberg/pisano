<script lang="ts">
	import '../global.css';
	import Titlebar from '../lib/Titlebar.svelte';
	import { browser } from '$app/environment';
	import { locale } from '$lib/i18n';

	if (browser) {
		import('@tauri-apps/api/window').then(({ getCurrent }) => {
			const win = getCurrent();
			win.show();
		});

		// locale.set('de-DE')
		locale.set(navigator.language)
	}
</script>

<Titlebar />
<main>
	<slot />
</main>
