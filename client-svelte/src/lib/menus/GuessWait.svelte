<script lang="ts">
	import { getGame, sleep } from '$lib/functions/requests';
	import { onMount } from 'svelte';
	export let setGameState: (new_state: string) => void;
	export let game_name: string | null;
	let question: string;
	let players: Array<string> = [];
	let rounds: Array<object>;

	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				console.log(data);
				players = data.players;
				rounds = data.rounds;
				let round = rounds.length - 1;
				if (data.rounds[round].guesses.length == players.length) {
					setGameState('wager');
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
</main>
