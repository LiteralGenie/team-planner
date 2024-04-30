<script lang="ts"></script>

<div class="hex outer-hex-color h-24 w-24 p-1">
    <div
        class="hex inner-hex-color h-full w-full flex justify-center items-center"
    >
        <button
            on:click
            type="button"
            class="h-full w-full text-5xl font-extralight"
        >
            +
        </button>
    </div>
</div>

<style lang="postcss">
    .outer-hex-color {
        background-color: hsl(var(--muted-foreground) / 30%);
    }

    .inner-hex-color {
        background-color: hsl(var(--background) / 90%);
        color: hsl(var(--muted-foreground));

        &:hover {
            background-color: hsl(var(--background) / 40%);
        }
    }

    .hex {
        /**
         * Draws hex in counter-clockwise fashion, starting from bottom corners
         *    ________
         *   /        \
         *  /          \
         * /            \
         * \            /
         *  \          /
         *   \________/
         *   x1      x2
         *
         * A regular hexagon has equal interior angles which works out to 120deg each
         * and assuming a side length of 1, that works out to
         * x1 = 0.5 or 25% of the total width (=2)
         * so x2 is (0.5 + 1) which is 75% of the total width
         */
        clip-path: polygon(
            25% 0%,
            75% 0%,
            100% 50%,
            75% 100%,
            25% 100%,
            0% 50%
        );
    }
</style>
