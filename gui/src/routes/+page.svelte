<script lang="ts">
    import loadWasm, * as tft from '$lib/assets/wasm/tft_core'

    loadWasm()
    window.tft = tft
</script>
