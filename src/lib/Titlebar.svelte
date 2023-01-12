<script lang="ts">
	import { withBoundingCircle } from '../stores/with-bounding-circle';
	import { sequence, SequenceName } from '../stores/sequence';
	import { t } from '$lib/i18n';
</script>

<header data-tauri-drag-region>
	<div style="grid-area: info;" class="vstack">
		<span style="font-weight: bold;">Pisano</span>
	</div>

	<label style="grid-area: bounding-circle;">
		{$t('titlebar.with_bounding_circle')}
		<input type="checkbox" bind:checked={$withBoundingCircle} />
	</label>

	<label style="grid-area: sequence;">
		<select bind:value={$sequence}>
			<option value={SequenceName.Fibonacci} default>{$t('titlebar.seq.fibonacci')}</option>
			<option value={SequenceName.Primes}>{$t('titlebar.seq.primes')}</option>
			<option value={SequenceName.Lucas}>{$t('titlebar.seq.lucas')}</option>
			<option value={SequenceName.Ramanujan}>{$t('titlebar.seq.ramanujan')}</option>
			<option value={SequenceName.SHC}>{$t('titlebar.seq.shc')}</option>
			<option value={SequenceName.Pronic}>{$t('titlebar.seq.pronic')}</option>
			<option value={SequenceName.Composite}>{$t('titlebar.seq.composite')}</option>
		</select>
	</label>
</header>

<style>
	:root {
		--titlebar-background: rgba(246, 241, 249, 0.9);
	}

	header {
		width: 100vw;
		position: fixed;
		height: 52px;
		display: grid;
		grid-template-columns: 80px 1fr 1fr 1fr 1fr;
		grid-template-areas: '. info . bounding-circle sequence';
		gap: 0.75em;
		align-items: center;
		transition: box-shadow 150ms, background-color 150ms;
		z-index: 100;
		box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.13);
		background-color: var(--titlebar-background);
		-webkit-backdrop-filter: blur(20px);
		backdrop-filter: blur(10px);
		font-size: 0.9rem;
		line-height: 1.1rem;
		padding: 0 10px;
		box-sizing: border-box;
	}

	.vstack {
		display: flex;
		flex-direction: column;
	}

	header > * {
		user-select: none;
		pointer-events: none;
	}

	select,
	input {
		pointer-events: all;
	}

	select {
		font-size: 1.2em;
	}

	select:focus {
		outline: none;
	}

	@media (prefers-color-scheme: dark) {
		:root {
			--titlebar-background: rgba(55, 51, 58, 0.9);
		}

		header {
			box-shadow: 0px 1px 1px 0px rgba(0, 0, 0, 0.2);
		}
	}
</style>
