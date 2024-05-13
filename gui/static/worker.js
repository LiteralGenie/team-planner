// https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html

importScripts('/wasm/tft_core.js')

const { TeamFinder } = wasm_bindgen

async function init_wasm_in_worker() {
    console.log('Initializing worker')
    await wasm_bindgen('/wasm/tft_core_bg.wasm')

    const finder = TeamFinder.new()

    let ctx = {}

    // Set callback to handle messages passed to the worker.
    self.onmessage = async (event) => {
        console.log('Got worker event', event.data)

        switch (event.data.type) {
            case 'setOptions':
                const { options, var_to_champion } = event.data

                finder.reset(options)

                ctx = { var_to_champion }

                return
            case 'nextSolution':
                const teamVars = finder.next()
                const teamIds = teamVars.map(
                    (v) => ctx.var_to_champion[v]
                )
                self.postMessage(teamIds)
                return
        }
    }
}

init_wasm_in_worker()
