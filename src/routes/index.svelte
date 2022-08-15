<script lang="ts">
	import Intersector from '$lib/Intersector.svelte';
	const moduli = [...Array(500).keys()];
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
