// https://rustwasm.github.io/wasm-bindgen/examples/console-log.html

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_2(a: &str, b: &str);
}
