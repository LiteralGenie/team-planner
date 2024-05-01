<script lang="ts">
    import Button from '../../components/ui/button/button.svelte'
    import * as Card from '../../components/ui/card'
    import Dialog from '../dialog/dialog.svelte'
    import { getFilterFormContext } from '../form-context/context'
    import FilterButton from './filter-button.svelte'

    let showDialog = false

    const { form } = getFilterFormContext()
    $: console.log('form', $form)

    function handleDialogOpen() {
        showDialog = true
    }

    function handleDialogClose() {
        showDialog = false
    }
</script>

<div>
    <Dialog
        open={showDialog}
        slotIndex={1}
        on:close={handleDialogClose}
    />

    <Card.Root
        class="w-full h-48 p-8 flex flex-col justify-center gap-4"
    >
        <form>
            <FilterButton on:click={handleDialogOpen} />
        </form>

        <section class="w-full flex justify-end">
            <Button class="w-24">Search -></Button>
        </section>
    </Card.Root>
</div>
