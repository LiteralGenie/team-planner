use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use wasm_bindgen::prelude::wasm_bindgen;

use crate::lib::data::{ ChampionId, GameData, TraitId };

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub champion_ids: Vec<ChampionId>,
    pub trait_counts: HashMap<TraitId, i32>,
}

impl Team {
    pub fn new(
        sat_lits: Vec<String>,
        var_to_id: &HashMap<i32, ChampionId>,
        data: &GameData
    ) -> Self {
        let champion_ids: Vec<String> = sat_lits
            .iter()
            // Ignore negatives ("~v1")
            .filter(|s| !s.starts_with("~"))
            // Parse to int ("v1" -> 1)
            .map(|s| s[1..].parse::<i32>().unwrap())
            // Parse to champion id (1 -> "blah")
            .map(|idx| var_to_id[&idx].clone())
            .collect();

        let champions = champion_ids
            .iter()
            .map(|id| data.champions[id].clone());

        let mut trait_counts = HashMap::new();
        for c in champions {
            for t in c.traits {
                let count = trait_counts
                    .entry(t)
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
