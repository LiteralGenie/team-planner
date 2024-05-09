<script lang="ts">
    export let src: string
    export let showHover = false
    export let variant: 'sm' | 'md' = 'sm'
</script>

<div
    class:sm={variant === 'sm'}
    class:md={variant === 'md'}
    class:hover-layer={showHover}
    class="hex"
>
    <div class="hex outline-layer">
        <div class="hex border-layer">
            <div class="hex bg-layer">
                <img class="h-[66%] w-[66%]" {src} />
            </div>
        </div>
    </div>
</div>

<style lang="postcss">
    .hex {
        @apply h-full w-full flex justify-center items-center;

        &.outline-layer,
        &.bg-layer {
            background-color: #232323;
        }

        &.border-layer {
            background-color: #6b6d6b;
        }

        &.hover-layer {
            padding: 2.5px;
            cursor: pointer;

            &:hover,
            &:focus {
                filter: drop-shadow(
                    0px 0px 20px rgba(255, 199, 46, 0.9)
                );
            }
        }
    }

    .sm {
        & .outline-layer {
            padding: 1px;
        }
        & .border-layer {
            padding: 1px;
        }
    }

    .md {
        & .outline-layer {
            padding: 2px;
        }
        & .border-layer {
            padding: 2px;
        }
    }
</style>
