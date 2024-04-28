use varisat::{ CnfFormula, ExtendFormula, Lit, Solver };

use crate::console::log;

use super::CnfBuilder;

pub type Solution = Vec<i32>;
pub type Cnf = Vec<Vec<i32>>;

// fn solve_once(cnf: Cnf) -> Option<Solution> {
//     match Certificate::try_from(cnf) {
//         Ok(Certificate::SAT(ans)) => { Some(ans) }
//         Ok(Certificate::UNSAT) => None,
//         Err(e) => panic!("s UNKNOWN; {}", e),
//     }
// }

// Lazy solution "generator"
pub struct AllSolver<'a> {
    pub builder: CnfBuilder,
    // pub formula: CnfFormula,
    pub solver: Solver<'a>,
}

impl AllSolver<'_> {
    pub fn new(builder: &CnfBuilder) -> Self {
        let mut solver = Solver::new();

        for clause in builder.dump().iter() {
            let with_varisat_lits: Vec<Lit> = clause
                .iter()
                .map(|id| Lit::from_dimacs(*id as isize))
                .collect();

            solver.add_clause(&with_varisat_lits);
        }

        Self { builder: builder.clone(), solver }
    }

    pub fn next(&mut self) -> Option<Solution> {
        let _ = self.solver.solve();

        match self.solver.model() {
            Some(model) => {
                // log!(
                //     "Adding clause {:?}",
                //     Vec::from_iter(
                //         model.iter().map(|lit| !lit.clone())
                //     )
                // );
                self.solver.add_clause(
                    &Vec::from_iter(
                        model.iter().map(|lit| !lit.clone())
                    )
                );

                return Some(
                    model
                        .iter()
                        .map(|lit| lit.to_dimacs() as i32)
                        .collect()
                );
            }
            None => {
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
        let mut solver = AllSolver::new(&builder);
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
