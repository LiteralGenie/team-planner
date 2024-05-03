<script lang="ts">
    import type {
        AttributeSlotControls,
        AttributeSlotValues
    } from '../form-context/types'
    import type { CheckboxData } from './checkbox-group/checkbox-group.svelte'
    import CheckboxGroup from './checkbox-group/checkbox-group.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: options = [
        {
            value: slotValues.damageType.ad,
            label: 'Physical',
            onChange: (v: boolean) => handleChange('ad', v)
        },

        {
            value: slotValues.damageType.ap,
            label: 'Magical',
            onChange: (v: boolean) => handleChange('ap', v)
        }
    ] satisfies CheckboxData[]

    function handleChange(key: 'ad' | 'ap', value: boolean) {
        const ctrl = slotControls.controls.damageType.controls[key]
        ctrl.onChange(value)
    }
</script>

<CheckboxGroup label="Damage Type" {options} />
