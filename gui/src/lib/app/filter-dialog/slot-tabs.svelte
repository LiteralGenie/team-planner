<script lang="ts" context="module">
    export type SlotIndex = 'global' | number
</script>

<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { getFilterFormContext } from '../form-context/context'
    import type { FilterForm } from '../form-context/types'

    export let slotIndex: SlotIndex

    const { form } = getFilterFormContext()

    const dispatch = createEventDispatcher()

    function getIndicatorOffset(
        slots: FilterForm['slots'],
        slotIndex: SlotIndex
    ): string {
        if (slotIndex === 'global') {
            return '0%'
        } else {
            let offsetPercent = (slotIndex + 1) / (slots.length + 1)
            offsetPercent = 100 * offsetPercent
            return `${offsetPercent}%`
        }
    }
</script>

<section class="h-full border-r divider-color">
    <h1 class="font-bold pb-1">Filters</h1>

    <hr class="mx-4 my-2" />

    <div class="relative flex">
        <!-- Tabs -->
        <div class="flex flex-col">
            <button
                class:active={slotIndex === 'global'}
                on:click={() => dispatch('tabclick', 'global')}
            >
                Global
            </button>

            {#each $form.slots as slot, idx}
                <button
                    on:click={() => dispatch('tabclick', idx)}
                    class:active={slotIndex === idx}
                >
                    Slot #{idx + 1}
                </button>
            {/each}
        </div>

        <!-- Active tab indicator -->
        <div
            style="--offset: {getIndicatorOffset(
                $form.slots,
                slotIndex
            )}"
            class="indicator"
        ></div>
    </div>
</section>

<style lang="postcss">
    h1,
    button {
        @apply w-full px-6 text-left;
    }

    button {
        @apply min-w-max text-sm h-8;

        &.active {
            @apply text-primary;
        }
    }

    .indicator {
        height: 2em;
        width: 0.125em;
        position: absolute;
        top: var(--offset);
        right: 0;
        transition: all 0.2s;
        background-color: hsl(var(--primary) / 75%);
    }
</style>
