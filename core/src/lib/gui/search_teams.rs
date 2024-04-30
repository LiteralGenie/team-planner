use serde::{ Deserialize, Serialize };
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue, UnwrapThrowExt };
use crate::lib::data::{ ChampionId, GameData };

use crate::console::log;
use crate::lib::sat::{ build_champion_constraints, SubgraphSolver };

use super::search_champions::{ ChampionFilter, filter_champions };
use super::team::Team;

#[derive(Debug, Serialize, Deserialize)]
struct SearchOptions {
    pub team_size: u8,
    pub champions: Option<Vec<ChampionFilter>>,
    pub debug: Option<bool>,
}

#[wasm_bindgen]
pub fn search_teams(options: JsValue) -> JsValue {
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

    to_value(&results).unwrap()
}
