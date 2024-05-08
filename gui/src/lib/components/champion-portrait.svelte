<script lang="ts">
    import type { CostTier } from '$lib/app/form-context/types'

    export let src: string
    export let cost: CostTier
    export let hideInnerBorder = false
</script>

<div
    class:common={cost === 1}
    class:uncommon={cost === 2}
    class:rare={cost === 3}
    class:epic={cost === 4}
    class:legendary={cost === 5}
    class="outer-border"
>
    <div class:inner-border={!hideInnerBorder}>
        <img {src} />
    </div>
</div>

<style lang="postcss">
    .outer-border {
        border: 1.5px solid;

        &.common {
            border-color: hsl(0, 0%, 50%);
        }
        &.uncommon {
            border-color: hsl(132, 61%, 45%);
        }
        &.rare {
            border-color: hsl(200, 85%, 45%);
        }
        &.epic {
            border-color: hsl(296, 85%, 45%);
        }
        &.legendary {
            border-color: hsl(38, 73%, 35%);
        }
    }

    .inner-border {
        border: 4px solid black;
    }
</style>
