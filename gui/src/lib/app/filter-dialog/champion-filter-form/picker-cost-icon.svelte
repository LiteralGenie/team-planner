<script lang="ts">
    import type { CostTier } from '$lib/app/form-context/types'
    import GoldIcon from '$lib/icons/gold-icon.svelte'

    export let cost: CostTier
</script>

<div class="text-muted-foreground flex gap-1 items-center">
    <GoldIcon class="h-3 w-3" />

    <span>{cost}</span>
</div>

<style lang="postcss">
</style>
