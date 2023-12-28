<script lang="ts">
import Join from "$lib/menus/Join.svelte"
import {
    onMount
} from "svelte";

let game_state: string | null;

let production_url: string = "https://weight-inquiries.onrender.com/api/v1/game/";
let test_url: string = "http://0.0.0.0:8172/api/v1/game/";
onMount(() => {
    if (localStorage.getItem("game_state") == "undefined") {
        setGameState("join");
    } else {
        loadGameState();
    }
    if (window.location.href == "http://localhost:5173/") {
        localStorage.setItem("base_server_path", test_url);
    } else {
        localStorage.setItem("base_server_path", production_url);
    }
})

function setGameState(new_state: string) {
    localStorage.setItem("game_state", new_state);
    game_state = new_state;
}

function loadGameState() {
    game_state = localStorage.getItem("game_state");
}
</script>

{#if game_state == "join"}
<Join setGameState={setGameState} />
{:else if game_state == "guess"}
guess
{:else if game_state == "wager"}
wager
{:else if game_state == "score"}
score
{/if}
