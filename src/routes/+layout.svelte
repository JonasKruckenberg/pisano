<script lang="ts">
	import '../global.css';
	import Titlebar from '../lib/Titlebar.svelte';
	import { browser } from '$app/environment';

	if (browser) {
		import('@tauri-apps/api/window').then(({ getCurrent }) => {
			const win = getCurrent();
			win.show();
		});
	}
</script>

<Titlebar />
<main>
	<slot />
</main>
