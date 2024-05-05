<script lang="ts">
    import apIcon from '$lib/assets/tft/misc/statmodsabilitypowericon.png'
    import adIcon from '$lib/assets/tft/misc/statmodsattackdamageicon.png'

    export let variant: 'ad' | 'ap'
</script>

{#if variant === 'ad'}
    <img src={adIcon} class="h-4 w-4" />
{:else}
    <img src={apIcon} class="h-4 w-4" />
{/if}
