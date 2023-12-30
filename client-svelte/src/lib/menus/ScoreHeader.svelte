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
	<h3>Scores</h3>
	<div class="score_cards">
		{#each score_map as [player, score]}
			<div class="score_card">
				<div>
					{player}:
				</div>
				<div>
					{score}
				</div>
			</div>
		{/each}
	</div>
</main>

<style>
	.score_cards {
		display: flex;
		padding: 0.1em 0em;
	}

	.score_card {
		padding: 0.6em 1em;
		line-height: 1.3em;
		appearance: none;
		background: none;
		font-weight: 600;
		font-size: 1em;
		color: var(--gray);
		border: 1px solid var(--gray);
		border-radius: 0;
		outline: none;
		cursor: pointer;
		text-align: center;
	}
</style>
