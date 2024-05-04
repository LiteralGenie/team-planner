<script lang="ts">
    import { setFilterFormContext } from '$lib/app/form-context/context'
    import { DEFAULT_FILTER_FORM } from '$lib/app/form-context/defaults'
    import Search from '$lib/app/search/search.svelte'
    import TRAITS from '$lib/assets/tft/tfttraits.json'

    setFilterFormContext(DEFAULT_FILTER_FORM, TRAITS)
</script>

<main class="h-full w-full p-4 flex flex-col gap-4">
    <Search />

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
