use std::collections::HashSet;

use serde::{ Deserialize, Serialize };
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue, UnwrapThrowExt };

use crate::lib::data::{ Champion, ChampionId, GameData, TraitId };

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampionFilter {
    ids: Option<Vec<ChampionId>>,
    cost: Option<Vec<u8>>,
    range: Option<Vec<u8>>,
    trait_ids: Option<Vec<TraitId>>,
    uses_ap: Option<bool>,
}

#[wasm_bindgen]
pub fn search_champions(filter: IChampionFilter) -> JsValue {
    let filter: ChampionFilter = serde_wasm_bindgen
        ::from_value(filter.into())
        .unwrap_throw();

    let champions = filter_champions(filter);

    to_value(&champions).unwrap()
}

pub fn filter_champions(filter: ChampionFilter) -> Vec<ChampionId> {
    // @TODO: Is there a way to avoid reinit'ing the champion data every call?
    //        While still exposing this function to JS. Cant wasm-bindgen return closures?
    let data = GameData::new();

    let mut champions: Vec<Champion> = Vec::from_iter(
        data.champions.into_values()
    );

    if let Some(ids) = filter.ids {
        let ids = HashSet::<ChampionId>::from_iter(ids.into_iter());
        champions = champions
            .into_iter()
            .filter(|c| ids.contains(&c.name))
            .collect();
    }

    if let Some(costs) = filter.cost {
        let costs = HashSet::<u8>::from_iter(costs.into_iter());
        champions = champions
            .into_iter()
            .filter(|c| costs.contains(&c.cost))
            .collect();
    }

    if let Some(ranges) = filter.range {
        let ranges = HashSet::<u8>::from_iter(ranges.into_iter());
        champions = champions
            .into_iter()
            .filter(|c| ranges.contains(&c.range))
            .collect();
    }

    if let Some(trait_ids) = filter.trait_ids {
        let trait_ids = HashSet::<TraitId>::from_iter(
            trait_ids.into_iter()
        );
        champions = champions
            .into_iter()
            .filter(|c|
                c.traits.iter().any(|t| trait_ids.contains(t))
            )
            .collect();
    }

    if let Some(uses_ap) = filter.uses_ap {
        champions = champions
            .into_iter()
            .filter(|c| c.uses_ap == uses_ap)
            .collect();
    }

    champions
        .iter()
        .map(|c| c.name.clone())
        .collect()
}

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str =
    r#"

interface IChampionFilter {
    team_size: number
    champions?: IChampionFilter[]
    debug?: boolean
}

type SearchTeams = (options: IChampionFilter) => Team[]

"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IChampionFilter")]
    pub type IChampionFilter;
}
