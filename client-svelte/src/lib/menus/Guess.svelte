<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { getGame, postGuess, sleep } from '$lib/functions/requests';
	import { onMount } from 'svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;
	let guess: string = '';

	function onClickSubmit() {
		if (guess == '') {
			return;
		}
		postGuess(game_name, name, parseInt(guess)).then((response) => {
			if (response.ok) {
				setGameState('guess_wait');
			}
		});
	}

	let question: string;
	let players: Array<string> = [];
	let rounds: Array<object>;

	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				rounds = data.rounds;
				question = data.rounds[rounds.length - 1].question.question;
			});
	}

	async function getGameLoop() {
		if (localStorage.getItem('game_state') == 'guess') {
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
	<h1>Make a guess with your brain.</h1>
	<div>
		{question}
	</div>
	<div>
		<InputField bind:value={guess} text="enter your guess here" />
		<Button text="submit" onClick={onClickSubmit} />
	</div>
</main>
