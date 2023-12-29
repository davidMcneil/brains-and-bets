<script lang="ts">
	import Button from '$lib/Button.svelte';
	import Guess from '$lib/menus/Guess.svelte';
	import GuessWait from '$lib/menus/GuessWait.svelte';
	import Join from '$lib/menus/Join.svelte';
	import Wager from '$lib/menus/Wager.svelte';
	import { onMount } from 'svelte';

	let game_state: string | null;

	let production_url: string = 'https://weight-inquiries.onrender.com/api/v1/game/';
	let test_url: string = 'http://0.0.0.0:8172/api/v1/game/';
	onMount(() => {
		if (!localStorage.getItem('game_state')) {
			setGameState('join');
		} else {
			loadGameState();
		}
		if (localStorage.getItem('name') == 'undefined') {
		}
		if (window.location.href == 'http://localhost:5173/') {
			localStorage.setItem('base_server_path', test_url);
		} else {
			localStorage.setItem('base_server_path', production_url);
		}
	});

	function setGameState(new_state: string) {
		localStorage.setItem('game_state', new_state);
		game_state = new_state;
	}

	function loadGameState() {
		game_state = localStorage.getItem('game_state');
	}

	function reset() {
		setGameState('join');
	}
</script>

{#if game_state == 'join'}
	<Join {setGameState} />
{:else if game_state == 'guess'}
	<Guess
		{setGameState}
		name={localStorage.getItem('name')}
		game_name={localStorage.getItem('game_name')}
	/>
{:else if game_state == 'guess_wait'}
	<GuessWait {setGameState} game_name={localStorage.getItem('game_name')} />
{:else if game_state == 'wager'}
	<Wager {setGameState} name={localStorage.getItem('name')} game_name={localStorage.getItem('game_name')} />
{:else if game_state == 'wager_wait'}
    wager wait
{:else if game_state == 'score'}
	score
{/if}

<div>
	<Button text="Reset" onClick={reset} />
</div>

<style>
	@import '../app.css';
</style>
