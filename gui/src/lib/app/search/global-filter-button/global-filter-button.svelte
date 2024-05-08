<script>
    import Button from '$lib/components/ui/button/button.svelte'

    import CogIcon from '$lib/icons/cog-icon.svelte'
</script>

<Button
    on:click
    class="settings-button text-foreground px-3 flex gap-2"
    variant="outline"
>
    <span class="flex gap-1 items-center">
        <CogIcon class="h-4 w-4" />
    </span>
</Button>
