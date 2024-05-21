<script lang="ts">
    import ChampionPortrait from '$lib/components/champion-portrait.svelte'
    import SpellTooltip from '$lib/components/spell-tooltip.svelte'
    import Button from '$lib/components/ui/button/button.svelte'
    import * as Popover from '$lib/components/ui/popover/index.js'
    import * as Tooltip from '$lib/components/ui/tooltip/index.js'

    export let id: string
</script>

<!-- Desktop version -->
<span class="h-12 w-12 hidden md:block">
    <Tooltip.Root
        group="spell"
        openDelay={100}
        closeOnPointerDown={true}
        disableHoverableContent={true}
    >
        <Tooltip.Trigger class="cursor-default">
            <ChampionPortrait {id} />
        </Tooltip.Trigger>
        <Tooltip.Content class="spell-tooltip-container">
            <SpellTooltip champion_id={id} />
        </Tooltip.Content>
    </Tooltip.Root>
</span>

<!-- Mobile version -->
<span class="h-12 w-12 block md:hidden">
    <Popover.Root>
        <Popover.Trigger asChild let:builder>
            <Button variant="ghost" builders={[builder]} class="p-0">
                <ChampionPortrait {id} hideInnerBorder />
            </Button>
        </Popover.Trigger>
        <Popover.Content class="spell-tooltip-container">
            <SpellTooltip champion_id={id} />
        </Popover.Content>
    </Popover.Root>
</span>
