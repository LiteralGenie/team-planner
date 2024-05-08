<script lang="ts">
    export let src: string
</script>

<div class="hex hover-fill p-[1px]">
    <div class="hex dark-fill p-[1px]">
        <div class="hex light-fill p-[1px]">
            <div class="hex dark-fill">
                <img class="h-[66%] w-[66%]" {src} />
            </div>
        </div>
    </div>
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
</style>
