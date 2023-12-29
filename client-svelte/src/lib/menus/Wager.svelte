<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { getGame, postWager, getScore } from '$lib/functions/requests';
	import { text } from '@sveltejs/kit';
	import { onMount } from 'svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;
	let wager_amount: string = '1';

	function onClickSubmit(guess: number | null) {
		postWager(game_name, name, guess, parseInt(wager_amount)).then((response) => {
			if (response.ok) {
				setGameState('wager_wait');
			}
		});
	}

	let players: Array<string> = [];
	let rounds: Array<object>;
    let current_round: object;
    let guesses: Array<object> = [];
    let my_score: number;

	async function readGameState() {
		getGame(game_name)
			.then((response) => response.json())
			.then((data) => {
				players = data.players;
				rounds = data.rounds;
                current_round = data.rounds[data.rounds.length - 1];
                guesses = current_round.guesses;
                guesses.sort();
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
		name: {name}
		game_name: {game_name}
	</div>
    <div>
        My score: {my_score}
    </div>
	<div>
		<InputField bind:value={wager_amount} text="enter your bet here" />
	</div>
	<div>Guess:</div>
	{#each guesses as guess}
		<div>
			{guess.player} {guess.guess}
		</div>
        <div>
            <Button text="{guess.guess}" onClick={() => onClickSubmit(guess.guess)}/>
        </div>
	{/each}
    <Button text="lower" onClick={() => onClickSubmit(null)} />
</main>
