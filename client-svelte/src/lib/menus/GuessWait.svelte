<script lang="ts">
	import { getGame, sleep } from '$lib/functions/requests';
	import { onMount } from 'svelte';
	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;
	let question: string;
	let players: Array<string> = [];
	let rounds: Array<object>;
	let waiting_for: Array<string> = [];

	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				rounds = data.rounds;
				let round = rounds.length - 1;
				if (data.rounds[round].guesses.length == players.length) {
					setGameState('wager');
				} else {
					waiting_for = players.filter(
						(player) => !data.rounds[round].guesses.some((guess) => guess.player === player)
					);
				}
				question = data.rounds[rounds.length - 1].question.question;
			});
	}

	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'guess_wait') {
			readGameState();
			await sleep(1000);
			getGameLoop();
		}
	}

	onMount(() => {
		getGameLoop();
	});
</script>

<main>
	<h1>Waiting for other players...</h1>
	<ul>
		{#each waiting_for as waiting_for_player}
			<li>{waiting_for_player}</li>
		{/each}
	</ul>
</main>
