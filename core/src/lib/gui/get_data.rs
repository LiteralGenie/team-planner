use serde_wasm_bindgen::to_value;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue };

use crate::lib::data::GameData;

#[wasm_bindgen]
pub fn get_data() -> JsValue {
    let data = GameData::new();
    to_value(&data).unwrap()
}

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str = r#"
type GetData = () => IGameData
"#;
