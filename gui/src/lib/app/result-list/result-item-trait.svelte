<script lang="ts" context="module">
</script>

<script lang="ts">
    import TraitIcon from '$lib/components/trait-icon.svelte'
    import TraitTooltip from '$lib/components/trait-tooltip.svelte'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'
    import { TRAITS_BY_ID } from '$lib/constants'
    import { type TraitLevel } from '../form-context/utils'

    export let id: string
    export let count: number
    export let level: TraitLevel | null

    $: trait = TRAITS_BY_ID[id]
</script>

<span
    class:inactive={level === null}
    class="inline-flex gap-0 justify-center items-center"
>
    <div class="icon">
        <Tooltip.Root
            openDelay={100}
            closeOnPointerDown={true}
            group="trait"
            disableHoverableContent={true}
        >
            <Tooltip.Trigger class="h-[2.5em] w-[2.5em]">
                <TraitIcon
                    id={trait.trait_id}
                    style={level?.style_name}
                    variant="md"
                />
            </Tooltip.Trigger>
            <Tooltip.Content class="boring-tooltip max-w-[400px]">
                <TraitTooltip
                    trait_id={trait.trait_id}
                    traitLevelIdx={level?.levelIdx}
                />
            </Tooltip.Content>
        </Tooltip.Root>
    </div>

    <span class="label w-6 text-xs flex justify-center items-center">
        {count}
    </span>
</span>

<style lang="postcss">
    .label {
        border-width: 1.5px;
        border-left: none;
        border-color: #6b6d6b;
        height: 1.55em;
        width: 2em;
    }

    .icon :global(.outline-layer) {
        padding-right: 0;
    }

    .inactive {
        display: none;

        @media screen(md) {
            display: flex;
        }
    }
</style>
