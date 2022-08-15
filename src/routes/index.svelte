<script lang="ts">
import { browser } from '$app/env';

	import Intersector from '$lib/Intersector.svelte';
import { onMount } from 'svelte';

	const moduli = [...Array(500).keys()];

    const updateStroke = () => stroke = getComputedStyle(document.documentElement).color

    let stroke: string
    
    // updateStroke() only works in a DOM env, so we have to dispatch it initially behind this guard
    if(browser) {
        updateStroke()
    }

    onMount(() => {
        const lightMode = matchMedia('(prefers-color-scheme: light)')

        lightMode.addEventListener("change", updateStroke)

        return () => lightMode.removeEventListener("change", updateStroke);
    })
</script>

<section class="circles">
	{#each moduli as modulus}
		<Intersector once={true} let:intersecting>
			{#if intersecting}
				<div class="circle">
					{modulus + 1}
					<img
						src={`circleplot://localhost?modulus=${modulus + 1}&stroke=${stroke}`}
						alt={`Circle plot for the Pisano period with modulus ${modulus}`}
					/>
				</div>
			{/if}
		</Intersector>
	{/each}
</section>

<style>
	.circles {
		width: 85vw;
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
		gap: 3.5rem;
	}

	.circle {
		text-align: start;
		font-family: Menlo, Consolas, Monaco, Liberation Mono, Lucida Console, monospace;
	}
</style>
