<script lang="ts">
	export let text: string;
	export let value = '';
</script>

<input bind:value placeholder={text} />
