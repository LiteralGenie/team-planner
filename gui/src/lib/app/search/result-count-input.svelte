<script lang="ts">
    import Svelecte from 'svelecte'
    import { getFilterFormContext } from '../form-context/context'

    const groupByOptions = [
        { value: 20, text: 20 },
        { value: 50, text: 50 },
        { value: 100, text: 100 },
        { value: 200, text: 200 },
        { value: 500, text: 500 }
    ]

    const { form, controls, submit } = getFilterFormContext()

    $: handleChange($form.resultCount)

    let rootEl: HTMLDivElement

    function handleChange(value: number) {
        controls.resultCount.onChange(value)
        submit()
    }

    function renderOption(
        item: any,
        isSelectionSection?: boolean
    ): string {
        return isSelectionSection
            ? `Result count: ${item.value}`
            : item.value.toString()
    }
</script>

<div bind:this={rootEl}>
    <Svelecte
        renderer={renderOption}
        on:change={(ev) => handleChange(ev.detail.value)}
        options={groupByOptions}
        value={$form.resultCount}
        searchable={false}
        keepSelectionInList={true}
        highlightFirstItem={false}
        clearable={false}
        class="min-w-max"
    />
</div>
