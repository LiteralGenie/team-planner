<script lang="ts">
    import FilterGroup from '$lib/components/app/filter-group.svelte'
    import Button from '$lib/components/ui/button/button.svelte'
    import * as Card from '$lib/components/ui/card'
</script>

<main class="h-full w-full p-4 flex flex-col gap-4">
    <Card.Root
        class="w-full h-48 p-4 flex flex-col justify-center items-center gap-4"
    >
        <FilterGroup />

        <FilterGroup />

        <section class="w-full flex justify-end">
            <Button class="w-24">Search -></Button>
        </section>
    </Card.Root>

    <div class="h-full rounded-md overflow-y-auto">
        <div class="h-full w-full p-4 flex flex-col">
            <div class="result">team 1</div>
            <hr class="divider" />
            <div class="result">team 2</div>
            <hr class="divider" />
            <div class="result">team 3</div>
            <hr class="divider" />
            <div class="result">team 4</div>
            <hr class="divider" />
            <div class="result">team 5</div>
            <hr class="divider" />
            <div class="result">team 6</div>
            <hr class="divider" />
            <div class="result">team 7</div>
            <hr class="divider" />
            <div class="result">team 8</div>
            <hr class="divider" />
            <div class="result">team 9</div>
            <hr class="divider" />
            <div class="result">team 10</div>
            <hr class="divider" />
            <div class="result">team 11</div>
            <hr class="divider" />
            <div class="result">team 12</div>
            <hr class="divider" />
            <div class="result">team 13</div>
            <hr class="divider" />
            <div class="result">team 14</div>
        </div>
    </div>
</main>

<style lang="postcss">
    .result {
        @apply min-h-24 flex flex-col justify-center;
    }

    .divider {
    }
</style>
