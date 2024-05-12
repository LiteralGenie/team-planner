<script lang="ts">
    import TraitIcon from '$lib/components/trait-icon.svelte'
    import { TRAITS_BY_ID } from '$lib/constants'

    export let id: string
</script>

<div
    class="w-20 py-4 text-muted-foreground flex flex-col items-center"
>
    <div class="h-12 w-12">
        <TraitIcon {id} showHover={true} variant="md" />
    </div>

    <span class="text-balance text-center break-all">
        {TRAITS_BY_ID[id].display_name}
    </span>
</div>

<style lang="postcss">
</style>
