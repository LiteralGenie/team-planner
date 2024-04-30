use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const WINDOW: &'static str =
    r#"


declare global {
    interface Window {
        tft: {
            search_champions: typeof search_champions
            search_teams: typeof search_teams
        }
    }
}

"#;
