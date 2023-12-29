<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { getGame, postWager, getScore } from '$lib/functions/requests';
	import { onMount } from 'svelte';
	import type { Guess } from '$lib/datatypes/Guess';
	import { compare } from '$lib/datatypes/Guess';
	import ButtonSet from '$lib/ButtonSet.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;
	let wager_amount: string = '1';
	let guess: number;

	function onClickSubmit() {
		if (guess == -1) {
			postWager(game_name, name, null, parseInt(wager_amount)).then((response) => {
				if (response.ok) {
					setGameState('wager_wait');
				}
			});
		}
		postWager(game_name, name, guess, parseInt(wager_amount)).then((response) => {
			if (response.ok) {
				setGameState('wager_wait');
			}
		});
	}

	let current_round: object;
	let guesses: Array<Guess> = [];
	let my_score: number;

	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				current_round = data.rounds[data.rounds.length - 1];
				current_round.guesses.forEach((guess) => {
					guesses.push(guess as Guess);
				});
				guesses.push({ player: 'lower', guess: -1 } as Guess);
				guesses = guesses.sort(compare);
			});
	}

	async function readScore() {
		getScore(game_name)
			.then((response) => response.json())
			.then((data) => {
				my_score = data[name];
			});
	}

	onMount(() => {
		readGameState();
		readScore();
	});
</script>

<main>
	<h1>Make a bet.</h1>
	<div>
		My score: {my_score}
	</div>
	<div>
		<InputField bind:value={wager_amount} text="enter your bet here" />
	</div>
	<!-- <div>Bet:</div> -->
	<!-- {#each guesses as guess}
		<div>
			<Button text={guess.guess} onClick={() => onClickSubmit(guess.guess)} />
			{guess.player}
		</div>
	{/each} -->
	<ButtonSet options={guesses} legend={'Select a guess:'} bind:userSelected={guess} />
	<div>
		<Button text="Submit" onClick={() => onClickSubmit(guess)} />
	</div>
	<div>
		{guess}
	</div>
</main>
