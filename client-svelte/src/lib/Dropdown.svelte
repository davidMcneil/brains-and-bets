<script lang="ts">
    export let options: Array<string>;
    export let selected: string;
</script>

<select bind:value={selected}>
    {#each options as option}
        <option value={option}>
            {option}
        </option>
    {/each}
</select>