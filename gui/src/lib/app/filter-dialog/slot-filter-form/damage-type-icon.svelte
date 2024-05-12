<script lang="ts">
    import { AD_ICON, AP_ICON } from '$lib/constants'

    export let variant: 'ad' | 'ap'
</script>

{#if variant === 'ad'}
    <img src={AD_ICON} class="h-4 w-4" />
{:else}
    <img src={AP_ICON} class="h-4 w-4" />
{/if}
