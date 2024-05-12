<script lang="ts">
    import { getFilterFormContext } from './form-context/context'

    const { formInitial } = getFilterFormContext()
</script>

{JSON.stringify($formInitial, null, 4)}
