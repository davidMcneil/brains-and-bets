<script lang="ts">
	import Button from '$lib/Button.svelte';
	import Range from '$lib/Range.svelte';
	import { getGame, postWager, getScore } from '$lib/functions/requests';
	import { onMount } from 'svelte';
	import type { Guess } from '$lib/datatypes/Guess';
	import { compare } from '$lib/datatypes/Guess';
	import ButtonSet from '$lib/ButtonSet.svelte';
	import NumberInputField from '$lib/NumberInputField.svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;
	let wager_amount: string = '1';
	let guess: number = null;

	function onClickSubmit() {
		postWager(game_name, name, guess, parseInt(wager_amount)).then((response) => {
			if (response.ok) {
				setGameState('wager_wait');
			}
		});
	}

	let current_round: object;
	let guesses: Array<Guess> = [];
	let my_score: number;
	let question: string;

	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				current_round = data.rounds[data.rounds.length - 1];
				current_round.guesses.forEach((guess) => {
					guesses.push(guess as Guess);
				});
				guesses.push({ player: 'lower', guess: null } as Guess);
				guesses = guesses.sort(compare);
				question = current_round.question.question;
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
	<h3>Make a wager no more than {my_score}</h3>
	<div style="padding-bottom: 1em;">
		<NumberInputField bind:value={wager_amount} text="enter your bet here"/>
	</div>
	<div>
		<Range min={1} max={my_score} initialValue={1} on:change={(e) => wager_amount = e.detail.value}/>
	</div>
	<div>{question}</div>
	<ButtonSet options={guesses} legend={'Select a guess:'} bind:userSelected={guess} />
	<div>
		<Button text="Submit" onClick={() => onClickSubmit(guess)} />
	</div>
</main>
