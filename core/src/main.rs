mod console;
mod lib;

use std::collections::HashSet;

use console::log;
use lib::sat::build_subgraph_contraints;
use splr::*;

fn main() {
    let builder = build_subgraph_contraints(
        4,
        3,
        HashSet::from_iter([
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
        ])
    );

    let cnf = builder.dump();

    match Certificate::try_from(cnf) {
        Ok(Certificate::SAT(ans)) => {
            let ans = ans
                .iter()
                .map(|id| builder.get_name(*id))
                .collect::<Vec<String>>()
                .join(", ");

            log!("s SATISFIABLE: {:?}", ans)
        }
        Ok(Certificate::UNSAT) => log!("s UNSATISFIABLE"),
        Err(e) => panic!("s UNKNOWN; {}", e),
    }
}
