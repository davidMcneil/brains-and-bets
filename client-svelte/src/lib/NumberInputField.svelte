<script lang="ts">
	export let text: string;
	export let value = '';
</script>

<input type="number" bind:value placeholder={text} />
