<script lang="ts">
    export let variant: 'default' | 'muted' = 'default'

    $: color =
        variant === 'default' ? '#c9aa6f' : 'hsl(var(--foreground))'
</script>

<svg
    viewBox="0 0 333 320"
    xmlns="http://www.w3.org/2000/svg"
    fill="transparent"
    {...$$props}
>
    <ellipse
        cx="133"
        cy="209"
        rx="120"
        ry="100"
        stroke={color}
        stroke-width="26"
    />

    <ellipse cx="203" cy="129" rx="130" ry="110" fill={color} />
</svg>
