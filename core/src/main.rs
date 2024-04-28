mod console;
mod lib;

use console::log;

use crate::lib::sat::{ build_kite_graph, SubgraphSolver };

fn main() {
    let constraints = build_kite_graph(4);

    log!(
        "Solving with {} constraints mapped to {} clauses",
        constraints.num_constraints,
        constraints.factory
            .cnf_of(constraints.formula)
            .to_string(&constraints.factory)
            .chars()
            .filter(|c| *c == '&')
            .count() + 1
    );

    let mut solver = SubgraphSolver::new(constraints);

    loop {
        match solver.next() {
            Some(model) => {
                log!("Found solution: {:?}", model);
            }
            None => {
                log!("No more solutions.");
                break;
            }
        }
    }
}
