<script lang="ts">
    export let text: string;
    export let onClick = () => (console.log("hello world"));
</script>

	
<button on:click={onClick}> 
    {text}
</button>