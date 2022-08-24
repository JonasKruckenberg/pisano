<script lang="ts">
	import '../global.css';
	import Titlebar from '../lib/Titlebar.svelte';
	import { browser } from '$app/env';

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

<!-- <style>
	main {
		max-width: 1280px;
		margin: 0 auto;
		padding: 2rem;
		text-align: center;
	}
</style> -->
