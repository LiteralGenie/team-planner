use std::collections::{ HashMap, HashSet };

use serde::{ Deserialize, Serialize };
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue, UnwrapThrowExt };
use crate::lib::data::{ Champion, ChampionId, GameData, TraitId };

use crate::console::log;
use crate::lib::sat::{ build_champion_constraints, SubgraphSolver };

use super::team::Team;

#[derive(Debug, Serialize, Deserialize)]
struct SearchOptions {
    pub team_size: u8,
    pub champions: Option<Vec<ChampionFilter>>,
    pub debug: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChampionFilter {
    ids: Option<Vec<ChampionId>>,
    cost: Option<Vec<u8>>,
    range: Option<Vec<u8>>,
    trait_ids: Option<Vec<TraitId>>,
    uses_ap: Option<bool>,
}

#[wasm_bindgen]
pub fn search_teams(options: JsValue) {
    let options: SearchOptions = serde_wasm_bindgen
        ::from_value(options)
        .unwrap_throw();

    let data = GameData::new();

    let champion_filters: Vec<Vec<ChampionId>> = options.champions
        .unwrap_or(vec![])
        .into_iter()
        .map(|filter| filter_champions(filter))
        .collect();

    let (constraints, index_to_id) = build_champion_constraints(
        options.team_size,
        champion_filters,
        &data
    );

    log!("solving with {} constraints", constraints.num_constraints);
    if options.debug.unwrap_or(false) {
        // This takes a few seconds to run
        log!(
            "{} CNF clauses were generated",
            constraints.factory
                .cnf_of(constraints.formula)
                .to_string(&constraints.factory)
                .chars()
                .filter(|c| *c == '&')
                .count() + 1
        );
    }

    let mut solver = SubgraphSolver::new(constraints);

    // @TODO: How to pass the generator to JS so it can display results as they're generated?
    //        The constraints struct cant be serialized because the SAT library objects cant be.
    //        Will rust let us create a global cache that persists between calls
    //           so that JS only needs a resume-id to generate more results?
    //        Or do we have to manually serialize the SAT stuff
    //           (by converting to a formula string or something)
    let mut results: Vec<Team> = vec![];
    for _ in 0..20 {
        match solver.next() {
            Some(sol) => {
                let team = Team::new(sol, &index_to_id, &data);
                log!("{:?}", team);
                results.push(team);
            }
            None => {
                break;
            }
        }
    }
}

#[wasm_bindgen]
pub fn search_champions(filter: JsValue) -> JsValue {
    let filter: ChampionFilter = serde_wasm_bindgen
        ::from_value(filter)
        .unwrap_throw();

    let champions = filter_champions(filter);

    to_value(&champions).unwrap()
}

fn filter_champions(filter: ChampionFilter) -> Vec<ChampionId> {
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
