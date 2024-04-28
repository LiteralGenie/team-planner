use splr::{
    Certificate,
    Config,
    Solver,
    SolverError,
    SolveIF,
    SatSolverIF,
};

use crate::console::log;

pub type Solution = Vec<i32>;
pub type Cnf = Vec<Vec<i32>>;

fn solve_once(cnf: Cnf) -> Option<Solution> {
    match Certificate::try_from(cnf) {
        Ok(Certificate::SAT(ans)) => { Some(ans) }
        Ok(Certificate::UNSAT) => None,
        Err(e) => panic!("s UNKNOWN; {}", e),
    }
}

// Lazy solution "generator"
pub struct AllSolver {
    solver: Solver,
}

impl AllSolver {
    pub fn new(cnf: Cnf) -> Self {
        let solver = Solver::try_from((
            Config::default(),
            cnf.as_ref(),
        )).unwrap();

        Self { solver }
    }

    // https://github.com/shnarazk/splr?tab=readme-ov-file#all-solutions-sat-solver-as-an-application-of-incremental_solver-feature
    pub fn next(&mut self) -> Option<Solution> {
        match self.solver.solve() {
            Ok(Certificate::SAT(ans)) => {
                let remapped = ans
                    .iter()
                    .map(|i| -i)
                    .collect::<Vec<i32>>();

                match self.solver.add_clause(remapped) {
                    Err(SolverError::Inconsistent) => {
                        println!(
                            "c no answer due to level zero conflict"
                        );
                        return None;
                    }
                    Err(e) => {
                        println!("s UNKNOWN; {:?}", e);
                        return None;
                    }
                    Ok(_) => self.solver.reset(),
                }

                return Some(ans);
            }
            Ok(Certificate::UNSAT) => {
                println!("s UNSATISFIABLE");
                return None;
            }
            Err(e) => {
                println!("s UNKNOWN; {}", e);
                return None;
            }
        }
    }
}

// @jank: Solutions are not unique (in terms of subgraph nodes)
//        since nodes can be picked in different orders.
//        And due to chronic laziness, most of these tests
//        are only checking that the output contains all valid solutions,
//        and not checking that the output contains *only* valid solutions.
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        lib::sat::{ build_subgraph_contraints, CnfBuilder },
    };

    use super::{ AllSolver, Solution };

    /**
     * a - b
     */
    fn build_ab_graph(subgraph_size: i32) -> CnfBuilder {
        build_subgraph_contraints(
            2,
            subgraph_size,
            HashSet::from_iter([(0, 1)])
        )
    }

    /**
     * a - b
     * |   |
     * c - d
     */
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

    /**
     *   a
     *  / \
     * b   c
     *  \ /
     *   d
     *   |
     *   e
     */
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

    fn collect(solver: &mut AllSolver) -> Vec<Vec<i32>> {
        let mut results = Vec::<Vec<i32>>::new();

        loop {
            match solver.next() {
                Some(ans) => {
                    results.push(ans);
                }
                None => {
                    break;
                }
            }
        }

        results
    }

    fn assert_contains(
        var_count: i32,
        actual: Vec<Solution>,
        expected: Vec<Solution>
    ) {
        let actual = HashSet::<Solution>::from_iter(
            actual
                .clone()
                .into_iter()
                .map(|sol| {
                    let mut only_vars = sol.clone();
                    only_vars.truncate(var_count as usize);
                    only_vars
                })
        );

        for sol in expected.iter() {
            assert!(
                actual.contains(sol),
                "Expected {:?} to be in {:?}",
                sol,
                actual
            );
        }
    }

    fn format_solution(
        solution: Vec<i32>,
        builder: &CnfBuilder
    ) -> String {
        solution
            .iter()
            .map(|id| builder.get_name(*id))
            .map(|name| format!("{: >3}", name))
            .collect::<Vec<String>>()
            .join(", ")
    }

    #[test]
    fn test_ab() {
        let builder = build_ab_graph(1);
        let mut solver = AllSolver::new(builder.dump());
        let sols = collect(&mut solver);

        println!(
            "cnf {:?}",
            builder
                .dump()
                .iter()
                .map(|x| format_solution(x.clone(), &builder))
                .collect::<Vec<String>>()
                .join("\n")
        );
        println!(
            "sol: {}",
            format_solution(sols[0].clone(), &builder)
        );

        assert_contains(2, sols, vec![vec![1], vec![2]])
    }
}
