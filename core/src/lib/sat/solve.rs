use std::collections::HashSet;

use varisat::{ ExtendFormula, Lit, Solver };

use crate::console::log;

use super::CnfBuilder;

pub type Solution = Vec<i32>;

// Lazy solution "generator"
pub struct AllSolver<'a> {
    pub builder: CnfBuilder,
    pub solver: Solver<'a>,

    /**
     * Sometimes we only care about a certain subset in the solution,
     *   and we only want one solution per unique subset
     * To filter for those unique solutions, the 1-based ids of
     *   each var in the subset should be included here.
     * Defaults to all variables.
     */
    pub vars_to_dedupe: HashSet<i32>,
}

impl AllSolver<'_> {
    pub fn new(builder: &CnfBuilder) -> Self {
        let mut solver = Solver::new();

        log!("dumping clauses");
        let d = builder.dump();

        log!("adding {} clauses", d.len());
        for clause in builder.dump().iter() {
            let with_varisat_lits: Vec<Lit> = clause
                .iter()
                .map(|id| Lit::from_dimacs(*id as isize))
                .collect();

            solver.add_clause(&with_varisat_lits);
        }

        let vars_to_dedupe = HashSet::from_iter(
            1..=builder.variables.len() as i32
        );

        log!("returning solver");
        Self {
            builder: builder.clone(),
            solver,
            vars_to_dedupe,
        }
    }

    pub fn next(&mut self) -> Option<Solution> {
        let _ = self.solver.solve();

        match self.solver.model() {
            Some(model) => {
                // The solution we found but filtered to only contain certain vars
                let to_ignore = Vec::from_iter(
                    model
                        .iter()
                        .filter(|lit|
                            self.vars_to_dedupe.contains(
                                &(lit.to_dimacs().abs() as i32)
                            )
                        )
                        .map(|lit| !lit.clone())
                );

                self.solver.add_clause(&to_ignore);

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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::lib::sat::{
        build_subgraph_contraints,
        utils::{ vec_vec_to_hash_hash, HashedClause },
        CnfBuilder,
    };

    use super::{ AllSolver, Solution };

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

    fn truncate(solution: Solution, node_count: i32) -> Solution {
        let mut sol = solution.clone();
        sol.truncate(node_count as usize);
        sol
    }

    fn filter_negatives(solution: Solution) -> Solution {
        solution
            .into_iter()
            .filter(|var| var.is_positive())
            .collect()
    }

    /**
     * Checks that the first N literals of
     *   each actual solution matches one of the expected solutions
     * Each expected solution should only contain positive literals.
     *   eg to check if the size-3 subgraph defined by nodes { 3, 5 } are in the solution,
     *      pass in [3, 5] or [5,3], NOT [-1, -2, 3, -4, 5]
     */
    fn assert_solutions(
        node_count: i32,
        actual: Vec<Solution>,
        expected: Vec<Solution>
    ) {
        let actual = vec_vec_to_hash_hash(
            actual
                .into_iter()
                .map(|sol| truncate(sol, node_count))
                .map(|sol| filter_negatives(sol))
                .collect()
        );

        let expected = vec_vec_to_hash_hash(
            expected
                .into_iter()
                .map(|sol| truncate(sol, node_count))
                .map(|sol| filter_negatives(sol))
                .collect()
        );

        assert_eq!(
            actual.len(),
            expected.len(),
            "Expected {} solutions but got {}: {:?}",
            expected.len(),
            actual.len(),
            actual
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
    fn test_ab1() {
        let (builder, node_count) = build_ab_graph(1);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(node_count, sols, vec![vec![1], vec![2]])
    }

    #[test]
    fn test_ab2() {
        let (builder, node_count) = build_ab_graph(2);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(node_count, sols, vec![vec![1, 2]])
    }

    #[test]
    fn test_square1() {
        let (builder, node_count) = build_square_graph(1);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            vec![vec![1], vec![2], vec![3], vec![4]]
        )
    }

    #[test]
    fn test_square2() {
        let (builder, node_count) = build_square_graph(2);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 4]]
        )
    }

    #[test]
    fn test_square3() {
        let (builder, node_count) = build_square_graph(3);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            // Solutions clockwise
            vec![
                vec![1, 2, 4],
                vec![2, 4, 3],
                vec![4, 3, 1],
                vec![3, 1, 2]
            ]
        )
    }

    #[test]
    fn test_square4() {
        let (builder, node_count) = build_square_graph(4);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            // Solutions clockwise
            vec![vec![1, 2, 3, 4]]
        )
    }

    #[test]
    fn test_kite1() {
        let (builder, node_count) = build_kite_graph(1);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        )
    }

    #[test]
    fn test_kite2() {
        let (builder, node_count) = build_kite_graph(2);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 4],
                vec![4, 5]
            ]
        )
    }

    #[test]
    fn test_kite3() {
        let (builder, node_count) = build_kite_graph(3);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            // Solutions clockwise
            vec![
                vec![1, 3, 4],
                vec![3, 4, 2],
                vec![4, 2, 1],
                vec![2, 1, 3],
                vec![2, 4, 5],
                vec![3, 4, 5]
            ]
        )
    }

    #[test]
    fn test_kite4() {
        let (builder, node_count) = build_kite_graph(4);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(
            node_count,
            sols,
            vec![
                vec![2, 3, 4, 5], // all but 1
                vec![1, 3, 4, 5], // all but 2
                vec![1, 2, 4, 5], // all but 3
                vec![1, 2, 3, 4] // all but 5
                // but removing 4 also removes 5, so a 4-graph can't be constructed
            ]
        )
    }

    #[test]
    fn test_kite5() {
        let (builder, node_count) = build_kite_graph(5);

        let mut solver = AllSolver::new(&builder);
        solver.vars_to_dedupe = HashSet::from_iter(1..=node_count);

        let sols = collect(&mut solver);

        assert_solutions(node_count, sols, vec![vec![1, 2, 3, 4, 5]])
    }
}
