<script lang="ts">
	import type { Guess } from '$lib/datatypes/Guess';

	export let options: Array<Guess>;
	export let legend: string;
	export let userSelected: number;
	export let fontSize = 16;
	export let flexDirection = 'column';

	const uniqueID = `legend-${Math.floor(Math.random() * 100)}`;

	const slugify = (str = '') => str.toLowerCase().replace(/ /g, '-').replace(/\./g, '');
</script>

<div
	role="radiogroup"
	class="group-container"
	aria-labelledby={`label-${uniqueID}`}
	style="font-size:{fontSize}px; flex-direction:{flexDirection}"
	id={`group-${uniqueID}`}
>
	<div class="legend" id={`label-${uniqueID}`}>{legend}</div>
	<div class="options">
		{#each options as { player, guess }}
			<div class="option">
				<input class="sr-only" type="radio" id={player} bind:group={userSelected} value={guess} />
				<label class="option" for={player}> {player + (guess ? ': ' + guess : '')} </label>
			</div>
		{/each}
	</div>
</div>

<style>
	/* @import "../app.css"; */
	:root {
		--accent-color: CornflowerBlue;
		--gray: #ccc;
	}
	.group-container {
		border-radius: 2px;
		border: 1px solid var(--gray-darker);
		display: flex;
		flex-direction: row;
	}

	.legend {
		margin-right: 0.5rem;
		font-weight: bold;
	}

	label {
		user-select: none;
		line-height: 1.2em;
		font-weight: 400;
		color: var(--gray);
		min-width: 100px;
		max-width: 150px;
		text-align: center;
	}

	.sr-only {
		position: absolute;
		clip: rect(1px, 1px, 1px, 1px);
		padding: 0;
		border: 0;
		height: 1px;
		width: 1px;
		overflow: hidden;
	}

	input[type='radio'] {
		position: absolute;
	}

	input[type='radio'] + label {
		position: relative;
	}

	.options {
		/* display: flex; */
		padding: 1em 0em;
	}

	.option {
		padding: 0.7em;
	}

	.option > label {
		padding: 0.6em 1em;
		line-height: 1.3em;
		appearance: none;
		background: none;
		font-weight: 600;
		font-size: 1.0em;
		color: var(--accent-color);
		border: 1px solid var(--accent-color);
		border-radius: 0;
		outline: none;
		cursor: pointer;
	}
	/* .option + .option > label {
		border-left-width: 0;
	}
	.option:first-of-type > label {
		border-radius: 6px 0 0 6px;
	}
	.option:last-of-type > label {
		border-radius: 0 6px 6px 0;
	} */

	input[type='radio']:checked + label {
		background: var(--accent-color);
		color: white;
	}

	input[type='radio']:focus + label {
		box-shadow: 0 0px 8px var(--accent-color);
	}

	input[type='radio']:disabled + label {
		color: darken(var(--gray), 10);
	}

	input[type='radio']:disabled + label {
		background: var(--gray);
	}
</style>
