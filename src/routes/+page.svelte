<script lang="ts">
	import { browser } from '$app/environment';
	import Intersector from '$lib/Intersector.svelte';
	import { onMount } from 'svelte';
	import { withBoundingCircle } from '../stores/with-bounding-circle';
	import { sequence } from '../stores/sequence';

	const moduli = [...Array(100).keys()];

	const updateStroke = () => (stroke = getComputedStyle(document.documentElement).color);

	let stroke: string;

	// updateStroke() only works in a DOM env, so we have to dispatch it initially behind this guard
	if (browser) {
		updateStroke();
	}

	onMount(() => {
		const lightMode = matchMedia('(prefers-color-scheme: light)');

		lightMode.addEventListener('change', updateStroke);

		return () => lightMode.removeEventListener('change', updateStroke);
	});
</script>

<section class="circles">
	{#each moduli as modulus}
		<Intersector once={true} let:intersecting>
			{#if intersecting}
				<div class="circle">
					{modulus + 1}
					<img
						src={`circleplot://localhost?sequence=${$sequence}&modulus=${
							modulus + 1
						}&stroke=${stroke}&with-bounding-circle=${$withBoundingCircle}`}
						alt={`Circle plot for the Pisano period with modulus ${modulus}`}
					/>
				</div>
			{/if}
		</Intersector>
	{/each}
</section>

<style>
	section {
		--padding-top: 62px;
		--padding-bottom: 3em;

		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(13rem, 1fr));
		gap: 7rem;

		height: calc(100vh - var(--padding-top) - var(--padding-bottom));
		overflow-y: scroll;
		scroll-behavior: smooth;
		padding: 0 3em;
		padding-top: var(--padding-top);
		padding-bottom: var(--padding-bottom);
	}

	.circle {
		text-align: start;
		font-family: Menlo, Consolas, Monaco, Liberation Mono, Lucida Console, monospace;
	}
</style>
