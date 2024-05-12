use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub champion_ids: Vec<u8>,
    pub trait_counts: HashMap<u8, u16>,
}

impl Team {
    pub fn new(
        sat_lits: Vec<String>,
        champion_traits: &HashMap<u8, Vec<u8>>
    ) -> Self {
        let champion_ids: Vec<u8> = sat_lits
            .iter()
            // Ignore negatives ("~v1")
            .filter(|s| !s.starts_with("~"))
            // Parse to int ("v1" -> 1)
            .map(|s| s[1..].parse::<u8>().unwrap())
            .collect();

        let mut trait_counts = HashMap::new();
        for c in champion_ids.iter() {
            let traits = champion_traits.get(c).unwrap();

            for t in traits.iter() {
                let count = trait_counts
                    .entry(*t)
                    .or_insert_with(|| 0);
                *count += 1;
            }
        }

        Self {
            champion_ids,
            trait_counts,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str =
    r#"

interface Team {
    champion_ids: string[]
    trait_counts: Record<string, number>    
}

"#;
