<script lang="ts">
    import loadWasm, * as tft from '$lib/assets/wasm/tft_core'
    import { onMount } from 'svelte'
    import '../app.css'

    onMount(() => {
        loadWasm()
        window.tft = tft
    })
</script>

<slot />
