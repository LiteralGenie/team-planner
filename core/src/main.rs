mod console;
mod lib;

use std::collections::HashSet;

use console::log;
use lib::sat::{ build_subgraph_contraints, AllSolver, CnfBuilder };

use crate::lib::sat::{ BooleanExpression, Literal, xor };

fn build_kite_graph(subgraph_size: i32) -> CnfBuilder {
    build_subgraph_contraints(
        5,
        subgraph_size,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
        ])
    )
}

fn build_ab_graph(subgraph_size: i32) -> CnfBuilder {
    build_subgraph_contraints(
        2,
        subgraph_size,
        HashSet::from_iter([(0, 1)])
    )
}

fn build_square_graph(subgraph_size: i32) -> CnfBuilder {
    build_subgraph_contraints(
        4,
        subgraph_size,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
        ])
    )
}

fn format_solution(
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
    let builder = build_ab_graph(1);

    log!("{:?}", builder.dump());
    let mut solver = AllSolver::new(&builder);

    loop {
        match solver.next() {
            Some(ans) => {
                log!("solution: {}", format_solution(ans, &builder));
            }
            None => {
                break;
            }
        }
    }
}
