mod console;
mod lib;

use std::collections::HashSet;

use console::log;
use lib::sat::{ build_subgraph_contraints, AllSolver, CnfBuilder };

/**
 * a - b
 */
fn build_ab_graph(subgraph_size: i32) -> (CnfBuilder, i32) {
    let node_count = 2;

    let builder = build_subgraph_contraints(
        node_count,
        subgraph_size,
        HashSet::from_iter([(0, 1)])
    );

    (builder, node_count)
}

/**
 * a - b
 * |   |
 * c - d
 */
fn build_square_graph(subgraph_size: i32) -> (CnfBuilder, i32) {
    let node_count = 4;

    let builder = build_subgraph_contraints(
        4,
        subgraph_size,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
        ])
    );

    (builder, node_count)
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
fn build_kite_graph(subgraph_size: i32) -> (CnfBuilder, i32) {
    let node_count = 5;

    let builder = build_subgraph_contraints(
        5,
        subgraph_size,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
        ])
    );

    (builder, node_count)
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
    log!("building constraints");
    let (builder, node_count) = build_kite_graph(5);

    log!("initing solver");
    let mut solver = AllSolver::new(&builder);
    solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

    loop {
        log!("solving");
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
