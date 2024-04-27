mod console;
mod lib;

use std::collections::HashSet;

use console::log;
use lib::sat::{ build_subgraph_contraints, CnfBuilder };
use splr::*;

/**
 * a - b
 */
fn build_ab_graph() -> CnfBuilder {
    build_subgraph_contraints(2, 2, HashSet::from_iter([(0, 1)]))
}

/**
 *   a
 *  / \
 * b   c
 *  \ /
 *   d
 *   |
 *   e
 */
fn build_kite_graph() -> CnfBuilder {
    build_subgraph_contraints(
        5,
        3,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
        ])
    )
}

/**
 * a - b
 * |   |
 * c - d
 */
fn build_square_graph() -> CnfBuilder {
    build_subgraph_contraints(
        4,
        3,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
        ])
    )
}

pub fn format_solution(
    solution: Vec<i32>,
    builder: &CnfBuilder
) -> String {
    solution
        .iter()
        .take(5)
        .map(|id| builder.get_name(*id))
        .map(|name| format!("{: >3}", name))
        .collect::<Vec<String>>()
        .join(", ")
}

fn main() {
    let builder = build_kite_graph();

    // let cnf = builder.dump();
    // let ans = solve_once(cnf);

    // match ans {
    //     Some(ans) => {
    //         log!(
    //             "Solution: {:?}",
    //             ans
    //                 .iter()
    //                 .map(|id| builder.get_name(*id))
    //                 .collect::<Vec<String>>()
    //                 .join(", ")
    //         )
    //     }
    //     None => log!("No solutions"),
    // }

    for ans in solve_all(&builder) {
        log!("Solution: {:?}", format_solution(ans, &builder));
    }
}

fn solve_once(cnf: Vec<Vec<i32>>) -> Option<Vec<i32>> {
    match Certificate::try_from(cnf) {
        Ok(Certificate::SAT(ans)) => { Some(ans) }
        Ok(Certificate::UNSAT) => None,
        Err(e) => panic!("s UNKNOWN; {}", e),
    }
}

fn solve_all(builder: &CnfBuilder) -> Vec<Vec<i32>> {
    log!("Solve all");
    let mut solver = Solver::try_from((
        Config::default(),
        builder.dump().as_ref(),
    )).unwrap();

    let mut results: Vec<Vec<i32>> = Vec::new();
    loop {
        match solver.solve() {
            Ok(Certificate::SAT(ans)) => {
                results.push(ans.clone());
                let ans = ans
                    .iter()
                    .map(|i| -i)
                    .collect::<Vec<i32>>();
                match solver.add_clause(ans) {
                    Err(SolverError::Inconsistent) => {
                        log!(
                            "c no answer due to level zero conflict"
                        );
                        break;
                    }
                    Err(e) => {
                        log!("s UNKNOWN; {:?}", e);
                        break;
                    }
                    Ok(_) => solver.reset(),
                }
            }
            Ok(Certificate::UNSAT) => {
                log!("s UNSATISFIABLE");
                break;
            }
            Err(e) => {
                log!("s UNKNOWN; {}", e);
                break;
            }
        }
    }

    results
}
