<script lang="ts">
    import { CHAMPIONS_BY_ID, CHAMPION_ICONS } from '$lib/constants'

    export let id: string
    export let hideInnerBorder = false

    $: cost = CHAMPIONS_BY_ID[id].tier
    $: src = CHAMPION_ICONS[id]
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
