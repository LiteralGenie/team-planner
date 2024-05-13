use std::collections::HashMap;
use web_time::Instant;

use serde::{ Deserialize, Serialize };
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue, UnwrapThrowExt };

use crate::console::log;
use crate::lib::sat::{ build_champion_constraints, SubgraphSolver };
use super::team::Team;

#[derive(Debug, Serialize, Deserialize)]
struct SearchOptions {
    pub team_size: u8,
    pub num_champions: u8,
    pub slots: Vec<Vec<u8>>,
    pub traits: HashMap<u8, Vec<String>>,

    pub debug: Option<bool>,
}

#[wasm_bindgen]
pub fn search_teams(options: ISearchTeamsOptions) -> JsValue {
    let start = Instant::now();

    let options: SearchOptions = serde_wasm_bindgen
        ::from_value(options.into())
        .unwrap_throw();

    log!("Searching for teams with options {:?}", options);

    let constraints = build_champion_constraints(
        options.num_champions,
        options.team_size,
        &options.slots,
        &options.traits
    );

    log!(
        "[{}ms] Solving with {} constraints",
        start.elapsed().as_millis(),
        constraints.num_constraints
    );
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
    for _ in 0..200 {
        match solver.next() {
            Some(sol) => {
                let team = Team::new(sol);

                if options.debug.unwrap_or(false) {
                    log!("{:?}", team);
                }

                results.push(team);
            }
            None => {
                break;
            }
        }
    }

    log!(
        "[{}ms] Returning {} teams",
        start.elapsed().as_millis(),
        results.len()
    );
    to_value(&results).unwrap()
}

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str =
    r#"

export interface ISearchTeamsOptions {
    team_size: number
    num_champions: number
    slots: Array<number[]>
    traits: Map<number, string[]>

    debug?: boolean
}

export type SearchTeams = (options: ISearchTeamsOptions) => Team[]

"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ISearchTeamsOptions")]
    pub type ISearchTeamsOptions;
}
