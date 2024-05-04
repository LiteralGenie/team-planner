<script lang="ts">
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'

    export let src: string
    export let label: string

    export let state: 'included' | 'excluded' | null
</script>

<div
    class="flex flex-col justify-center items-center text-center gap-[1px]"
>
    <button
        on:click
        type="button"
        class="h-12 w-12 relative select-none"
    >
        <!-- @todo trait description tooltip -->
        <!-- Hex icon -->
        <div class="hex hover-fill p-[2px]">
            <div class="hex dark-fill p-[2px]">
                <div class="hex light-fill p-[2px]">
                    <div class="hex dark-fill">
                        <img class="h-[66%] w-[66%]" {src} />
                    </div>
                </div>
            </div>
        </div>

        <!-- Selection indicator -->
        {#if state === 'included' || state === 'excluded'}
            <div
                class:green={state === 'included'}
                class:red={state === 'excluded'}
                class="absolute bottom-[0px] right-[0px] rounded-full p-[2px] text-foreground"
            >
                {#if state === 'included'}
                    <CheckmarkIcon class="h-3 w-3" />
                {:else if state === 'excluded'}
                    <XIcon class="h-3 w-3" />
                {/if}
            </div>
        {/if}
    </button>

    <!-- Label -->
    <span class="text-xs">{label}</span>
</div>

<style lang="postcss">
    .hex {
        @apply h-full w-full flex justify-center items-center;

        &.dark-fill {
            background-color: #232323;
        }

        &.light-fill {
            background-color: #6b6d6b;
        }

        &.hover-fill:hover {
            filter: drop-shadow(0px 0px 20px rgba(255, 199, 46, 0.9));
        }
    }

    .green {
        background-color: #39b549;
    }

    .red {
        background-color: #eb1a26;
    }
</style>
