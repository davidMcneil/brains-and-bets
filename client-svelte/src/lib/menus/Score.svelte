<script lang="ts">
	import Button from '$lib/Button.svelte';
	import InputField from '$lib/InputField.svelte';
	import { getGame, postWager, getScore } from '$lib/functions/requests';
	import { text } from '@sveltejs/kit';
	import { onMount } from 'svelte';

	export let setGameState: (new_state: string) => void;
	export let name: string | null;
	export let game_name: string | null;

	function onClickContinue() {
        setGameState("guess");
	}

    let score_map: Map<string, number> = new Map();
	
    async function readScore() {
		getScore(game_name)
			.then((response) => response.json())
			.then((data) => {
                for (var property in data) {
                    score_map  = score_map.set(property, data[property]);
                }
			});
	}

	onMount(() => {
        readScore();        
	});
</script>

<main>
	<h1>Score</h1>
	<div>
		name: {name}
		game_name: {game_name}
	</div>
	{#each score_map as [player, score]}
		<div>
			{player} {score}
		</div>
	{/each}
    <Button text="Continue" onClick={onClickContinue} />
</main>
