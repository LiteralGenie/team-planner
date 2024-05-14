use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub champion_ids: Vec<u8>,
}

impl Team {
    pub fn new(sat_lits: Vec<String>) -> Self {
        let champion_ids: Vec<u8> = sat_lits
            .iter()
            // Ignore negatives ("~v1")
            .filter(|s| !s.starts_with("~"))
            // Parse to int ("v1" -> 1)
            .map(|s| s[1..].parse::<u8>().unwrap())
            .collect();

        Self {
            champion_ids,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str =
    r#"

interface Team {
    champion_ids: number[]
}

"#;
