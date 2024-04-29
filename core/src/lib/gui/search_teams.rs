use serde::{ Deserialize, Serialize };
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue, UnwrapThrowExt };

use crate::console::log;

#[derive(Debug, Serialize, Deserialize)]
struct SearchOptions {
    pub team_size: i32,
}

#[wasm_bindgen]
pub fn search_teams(options: JsValue) {
    let options: SearchOptions = serde_wasm_bindgen
        ::from_value(options)
        .unwrap_throw();

    log!("options {:?}", options)
}
