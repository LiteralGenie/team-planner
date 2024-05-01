<script lang="ts">
    import type { FormControl } from '../form-context/form-control'

    export let control: FormControl<Boolean>
    export let tier: number

    let inputEl: HTMLInputElement

    $: id = `tier-${tier}`

    function handleChange() {
        control.onChange(inputEl.checked)
    }
</script>

<div>
    <input
        bind:this={inputEl}
        on:change={handleChange}
        type="checkbox"
        {id}
        name={id}
        checked
    />
    <!-- @TODO: COIN ICON -->
    <label for={id}> {tier + 1} cost </label>
</div>
