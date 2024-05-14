<script lang="ts">
    import { range } from 'radash'
    import Svelecte from 'svelecte'
    import { getFilterFormContext } from '../form-context/context'
    import { MAX_TEAM_SIZE } from '../form-context/defaults'

    const groupByOptions = [...range(2, MAX_TEAM_SIZE)].map(
        (idx) => ({ value: idx, text: idx })
    )

    const { form, controls, submit } = getFilterFormContext()

    let rootEl: HTMLDivElement

    function handleChange(value: number) {
        controls.teamSize.onChange(value)
        submit()
    }

    function renderOption(
        item: any,
        isSelectionSection?: boolean
    ): string {
        return isSelectionSection
            ? `<div class="flex"><span>Team size:</span> <span class="w-4 flex justify-end">${item.value}</span></div>`
            : item.value.toString()
    }
</script>

<div bind:this={rootEl} class="dark-select">
    <Svelecte
        renderer={renderOption}
        on:change={(ev) => handleChange(ev.detail.value)}
        options={groupByOptions}
        value={$form.teamSize}
        searchable={false}
        keepSelectionInList={true}
        highlightFirstItem={false}
        clearable={false}
        class="min-w-max"
    />
</div>

<style lang="postcss">
    .dark-select {
        --sv-bg: hsl(var(--background) / 90%);
        --sv-dropdown-active-bg: hsl(var(--card) / 100%);
        --sv-dropdown-selected-bg: hsl(var(--card) / 100%);
    }
    .dark-select :global(.sv-control--selection) {
        padding-right: 0 !important;
    }
</style>
