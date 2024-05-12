<script lang="ts" context="module">
</script>

<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'

    export let ids: string[]
</script>

<div class="card">
    <div class="flex justify-start gap-2">
        {#each ids as id}
            <span class="h-12 w-12">
                <ChampionPortrait {id} />
            </span>
        {/each}
    </div>
</div>

<style lang="postcss">
    .card {
        @apply p-4 rounded-sm;

        background-color: hsl(var(--card) / 60%);
    }
</style>
