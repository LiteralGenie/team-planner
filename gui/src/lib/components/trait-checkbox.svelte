<script lang="ts">
    import CheckmarkIcon from '$lib/icons/checkmark-icon.svelte'
    import XIcon from '$lib/icons/x-icon.svelte'

    export let src: string
    export let label: string

    export let state: 'included' | 'excluded' | null
</script>

<div
    class="root flex flex-col justify-center items-center text-center gap-[1px]"
    class:active={state !== null}
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
                class="mark absolute bottom-[2px] right-[3px] rounded-full p-[2px] text-foreground"
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
    <span class="text-xs text-muted-foreground">{label}</span>
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
    }

    .mark {
        /* Prevent hover effect from disappearing on icon hover */
        pointer-events: none;

        &.green {
            background-color: #39b549;
        }
        &.red {
            background-color: #eb1a26;
        }
    }

    /* Highlight on hover / selection */
    button {
        transition: all 0.2s;

        &:hover .hover-fill {
            filter: drop-shadow(0px 0px 20px rgba(255, 199, 46, 0.9));
        }
    }
    .root:not(.active) button {
        opacity: 0.3;

        &:hover {
            opacity: 0.5;
        }
    }
    .active span {
        @apply text-foreground;
    }
</style>
