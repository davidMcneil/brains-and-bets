<script lang="ts">
	import { getGame, sleep } from '$lib/functions/requests';
	import { onMount } from 'svelte';

	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;

	let rounds: Array<object>;
	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				let round = data.rounds.length - 1;
				if (data.rounds[round].wagers.length == 0) {
					setGameState('score');
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
</main>
