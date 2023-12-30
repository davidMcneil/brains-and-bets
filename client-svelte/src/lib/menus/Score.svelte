<script lang="ts">
	import Button from '$lib/Button.svelte';
	import { getGame, getScore, getRoundScore } from '$lib/functions/requests';
	import { onMount } from 'svelte';
	import { type Guess, compare } from '$lib/datatypes/Guess';
	import { getClosestGuess } from '$lib/functions/helpers';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	function onClickContinue() {
		setGameState('guess');
	}

	let score_map: Map<string, number> = new Map();
	let round_score_map: Map<string, number> = new Map();
	let question: string;
	let answer: string;
	let guesses: Array<Guess>;
	let closest_guess: Guess | null;

	async function readScore() {
		getScore(game_name)
			.then((response) => response.json())
			.then((data) => {
				for (var property in data) {
					score_map = score_map.set(property, data[property]);
				}
				score_map = new Map([...score_map.entries()].sort((a, b) => b[1] - a[1]));
			});
	}

	async function readRoundScore() {
		getRoundScore(game_name)
			.then((response) => response.json())
			.then((data) => {
				for (var property in data) {
					round_score_map = round_score_map.set(property, data[property]);
				}
			});
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				question = data.rounds[data.rounds.length - 2].question.question;
				answer = data.rounds[data.rounds.length - 2].question.answer;
				guesses = data.rounds[data.rounds.length - 2].guesses;
				guesses = guesses.sort(compare);
				closest_guess = getClosestGuess(guesses, parseInt(answer));
			});
	}

	onMount(() => {
		readScore();
		readRoundScore();
		readGame();
	});
</script>

<main>
	<h2>Scores</h2>
	{#each score_map as [player, score]}
		<div>
			{player}:
			{score}
			(change in score:
			{round_score_map.get(player) + (round_score_map.get(player) > 0 ? ' ✅' : ' ❌')})
		</div>
	{/each}

	<h2>Question and Answer</h2>
	<div>
		{question}
		<h3>{answer}</h3>
	</div>

	<h2>Closest Guess</h2>
	<div>
		{#if closest_guess == null}
			Everybody guessed over the real answer.
		{:else}
			{closest_guess.player} got the closest guess with {closest_guess.guess}
		{/if}
	</div>

	<Button text="Continue" onClick={onClickContinue} />
</main>
