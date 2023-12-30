<script lang="ts">
	import { getGame, getScore, sleep } from '$lib/functions/requests';
	import { onMount } from 'svelte';

	export let name: string | null;
	export let game_name: string | null;

	let score_map: Map<string, number> = new Map();
	let round_score_map: Map<string, number> = new Map();
	let question: string;
	let answer: string;

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

	async function readScoreLoop() {
		readScore();
		await sleep(1000);
		readScoreLoop();
	}

	async function readGame() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				question = data.rounds[data.rounds.length - 2].question.question;
				answer = data.rounds[data.rounds.length - 2].question.answer;
			});
	}

	onMount(() => {
		readScoreLoop();
		// readGame();
	});
</script>

<main>
	<div>Scores:</div>
	{#each score_map as [player, score]}
		<div>
			{player}:
			{score}
		</div>
	{/each}
</main>
