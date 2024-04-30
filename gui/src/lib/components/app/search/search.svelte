<script lang="ts">
    import Button from '../../ui/button/button.svelte'
    import * as Card from '../../ui/card'
    import FilterButton from './filter-button.svelte'
</script>

<Card.Root class="w-full h-48 p-8 flex flex-col justify-center gap-4">
    <form>
        <FilterButton />
    </form>

    <section class="w-full flex justify-end">
        <Button class="w-24">Search -></Button>
    </section>
</Card.Root>
