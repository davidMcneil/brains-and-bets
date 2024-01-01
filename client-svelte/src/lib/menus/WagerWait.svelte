<script lang="ts">
	import { getGame, sleep } from '$lib/functions/requests';
	import { onMount } from 'svelte';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;
	let players: Array<string> = [];
	let waiting_for: Array<string> = [];

	let rounds: Array<object>;
	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				let round = data.rounds.length - 1;
				if (data.rounds[round].wagers.length == 0) {
					setGameState('score');
				} else {
					waiting_for = players.filter(
						(player) => !data.rounds[round].wagers.some((wager) => wager.player === player)
					);
				}
			});
	}

	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'wager_wait') {
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
