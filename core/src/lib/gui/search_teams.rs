use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::Worker;
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
pub struct TeamFinder {
    options: SearchOptions,
    solver: SubgraphSolver,
}

#[wasm_bindgen]
impl TeamFinder {
    pub fn new() -> Self {
        let options = SearchOptions {
            team_size: 1,
            num_champions: 1,
            slots: vec![],
            traits: HashMap::new(),
            debug: Some(false),
        };

        let solver = init_solver(&options);

        Self {
            options,
            solver,
        }
    }

    pub fn reset(&mut self, options: JsValue) {
        let options: SearchOptions = serde_wasm_bindgen
            ::from_value(options.into())
            .unwrap_throw();

        self.solver = init_solver(&options);

        self.options = options;
    }

    pub fn next(&mut self) -> JsValue {
        let start = Instant::now();

        match self.solver.next() {
            Some(sol) => {
                let team = Team::new(sol);

                log!(
                    "[{}ms] Found solution",
                    start.elapsed().as_millis()
                );
                if self.options.debug.unwrap_or(false) {
                    log!("{:?}", team);
                }

                return to_value(&team.champion_ids).unwrap();
            }
            None => {
                return JsValue::null();
            }
        }
    }
}

fn init_solver(options: &SearchOptions) -> SubgraphSolver {
    let start = Instant::now();

    log!("Setting solver options {:?}", options);

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

    SubgraphSolver::new(constraints)
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
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ISearchTeamsOptions")]
    pub type ISearchTeamsOptions;
}
