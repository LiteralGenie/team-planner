use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const WINDOW: &'static str =
    r#"


declare global {
    interface Window {
        tft: {
            search_teams: SearchTeams
        }
    }
}

"#;
