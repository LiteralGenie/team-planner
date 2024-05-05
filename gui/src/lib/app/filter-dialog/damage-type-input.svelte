<script lang="ts">
    import type {
        AttributeSlotControls,
        AttributeSlotValues
    } from '../form-context/types'
    import type { CheckboxData } from './checkbox-group/checkbox-group.svelte'
    import CheckboxGroup from './checkbox-group/checkbox-group.svelte'
    import DamageTypeIcon from './damage-type-icon.svelte'

    export let slotControls: AttributeSlotControls
    export let slotValues: AttributeSlotValues

    $: options = [
        {
            value: slotValues.damageType.ad,
            label: 'Physical',
            onChange: (v: boolean) => handleChange('ad', v),
            prefix: DamageTypeIcon,
            prefixOpts: {
                variant: 'ad'
            }
        },

        {
            value: slotValues.damageType.ap,
            label: 'Magical',
            onChange: (v: boolean) => handleChange('ap', v),
            prefix: DamageTypeIcon,
            prefixOpts: {
                variant: 'ap'
            }
        }
    ] satisfies CheckboxData[]

    function handleChange(key: 'ad' | 'ap', value: boolean) {
        const ctrl = slotControls.controls.damageType.controls[key]
        ctrl.onChange(value)
    }
</script>

<CheckboxGroup label="Damage Type" {options} />
