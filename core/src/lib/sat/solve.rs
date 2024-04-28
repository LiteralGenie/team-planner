use logicng::{
    formulas::{ ToFormula, Variable },
    solver::minisat::{ sat::{ mk_lit, MsLit, MsVar }, MiniSat },
};

use super::SubgraphConstraints;

pub type Solution = Vec<String>;

/**
 * Reimplements logicng::solver::functions::enumerate_models
 * to make it lazy / generator-like
 */
pub struct SubgraphSolver {
    pub constraints: SubgraphConstraints,
    pub solver: MiniSat,
    pub solution_variables: Vec<Variable>,
}

impl SubgraphSolver {
    pub fn new(constraints: SubgraphConstraints) -> Self {
        let mut solver = MiniSat::new();
        solver.add(constraints.formula, &constraints.factory);

        let solution_variables = (0..constraints.num_vertices)
            .map(|i| format!("v{}", i))
            .map(|name| constraints.factory.var(&name))
            .collect();

        Self {
            constraints,
            solver,
            solution_variables,
        }
    }

    pub fn next(&mut self) -> Option<Solution> {
        self.solver.sat();

        let model = self.solver.model(Some(&self.solution_variables));

        match model {
            Some(model) => {
                self.block_latest_model();

                return Some(
                    Vec::from_iter(
                        model
                            .literals()
                            .iter()
                            .map(|lit|
                                lit.to_string(
                                    &self.constraints.factory
                                )
                            )
                    )
                );
            }
            None => {
                return None;
            }
        }
    }

    // https://github.com/booleworks/logicng-rs/blob/2fc0f76558fb9194cdf8a44b4c67a243116ea61c/src/solver/functions/model_enumeration.rs#L268
    fn block_latest_model(&mut self) {
        let relevant_indices: Vec<MsVar> = self.solution_variables
            .iter()
            .filter_map(|&v|
                self.solver.underlying_solver.idx_for_variable(v)
            )
            .collect();

        let model_from_solver = &self.solver.underlying_solver.model;

        let mut blocking_clause = Vec::<MsLit>::with_capacity(
            relevant_indices.len()
        );

        for var_index in relevant_indices {
            blocking_clause.push(
                mk_lit(var_index, model_from_solver[var_index.0])
            );
        }

        self.solver.underlying_solver.add_clause(
            blocking_clause,
            &None
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{ collections::HashSet, hash::{ Hash, Hasher } };

    use crate::lib::sat::{
        build_ab_graph,
        build_kite_graph,
        build_square_graph,
        build_subgraph_contraints,
        SubgraphConstraints,
    };

    use super::{ SubgraphSolver, Solution };

    // https://stackoverflow.com/questions/36562419/hashset-as-key-for-other-hashset
    #[derive(Debug)]
    pub struct HashedSolution(pub HashSet<String>);

    impl PartialEq for HashedSolution {
        fn eq(&self, other: &HashedSolution) -> bool {
            self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
        }
    }

    impl Eq for HashedSolution {}

    impl Hash for HashedSolution {
        fn hash<H>(&self, state: &mut H) where H: Hasher {
            let mut a: Vec<&String> = self.0.iter().collect();
            a.sort();
            for s in a.iter() {
                s.hash(state);
            }
        }
    }

    impl HashedSolution {
        pub fn from_vec(clause: &Solution) -> Self {
            Self(HashSet::from_iter(clause.clone().into_iter()))
        }
    }

    pub fn vec_vec_to_hash_hash(
        cnf: Vec<Solution>
    ) -> HashSet<HashedSolution> {
        let result = HashSet::from_iter(
            cnf
                .clone()
                .into_iter()
                .map(|clause| {
                    let hashed = HashedSolution::from_vec(&clause);

                    // No dupes should be removed, each variable should appear at most once
                    assert_eq!(
                        hashed.0.len(),
                        clause.len(),
                        "Duplicate variables removed from cnf clause {:?}",
                        clause
                    );

                    hashed
                })
        );

        // No dupes should be removed, each variable should appear at most once
        assert_eq!(
            result.len(),
            cnf.len(),
            "Duplicate clauses removed from {:?}",
            cnf
        );

        result
    }

    fn collect(solver: &mut SubgraphSolver) -> Vec<Solution> {
        let mut results = Vec::<Solution>::new();

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

    fn truncate(solution: Solution, num_vertices: &i32) -> Solution {
        let mut sol = solution.clone();
        sol.truncate(*num_vertices as usize);
        sol
    }

    fn filter_negatives(solution: Solution) -> Solution {
        solution
            .into_iter()
            .filter(|lit| !lit.starts_with("~"))
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
        num_vertices: &i32,
        actual: Vec<Solution>,
        expected: Vec<Solution>
    ) {
        let actual = vec_vec_to_hash_hash(
            actual
                .into_iter()
                .map(|sol| truncate(sol, num_vertices))
                .map(|sol| filter_negatives(sol))
                .collect()
        );

        let expected = vec_vec_to_hash_hash(
            expected
                .into_iter()
                .map(|sol| truncate(sol, num_vertices))
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

    fn to_id(id: i32) -> String {
        format!("v{}", id)
    }

    fn to_ids(ids: Vec<i32>) -> Vec<String> {
        Vec::from_iter(ids.iter().map(|id| to_id(*id)))
    }

    fn to_sols(ids: Vec<Vec<i32>>) -> Vec<Vec<String>> {
        Vec::from_iter(ids.into_iter().map(|sol| to_ids(sol)))
    }

    #[test]
    fn test_ab1() {
        let constraints = build_ab_graph(1);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(&nv, sols, to_sols(vec![vec![0], vec![1]]))
    }

    #[test]
    fn test_ab2() {
        let constraints = build_ab_graph(2);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(&nv, sols, to_sols(vec![vec![0, 1]]))
    }

    #[test]
    fn test_square1() {
        let constraints = build_square_graph(1);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            to_sols(vec![vec![0], vec![1], vec![2], vec![3]])
        )
    }

    #[test]
    fn test_square2() {
        let constraints = build_square_graph(2);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            to_sols(
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]
            )
        )
    }

    #[test]
    fn test_square3() {
        let constraints = build_square_graph(3);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            // Solutions clockwise
            to_sols(
                vec![
                    vec![0, 1, 3],
                    vec![1, 3, 2],
                    vec![3, 2, 0],
                    vec![2, 0, 1]
                ]
            )
        )
    }

    #[test]
    fn test_square4() {
        let constraints = build_square_graph(4);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            // Solutions clockwise
            to_sols(vec![vec![0, 1, 2, 3]])
        )
    }

    #[test]
    fn test_kite1() {
        let constraints = build_kite_graph(1);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            to_sols(vec![vec![0], vec![1], vec![2], vec![3], vec![4]])
        )
    }

    #[test]
    fn test_kite2() {
        let constraints = build_kite_graph(2);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            to_sols(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 3],
                    vec![3, 4]
                ]
            )
        )
    }

    #[test]
    fn test_kite3() {
        let constraints = build_kite_graph(3);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            // Solutions clockwise
            to_sols(
                vec![
                    vec![0, 2, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 0],
                    vec![1, 0, 2],
                    vec![1, 3, 4],
                    vec![2, 3, 4]
                ]
            )
        )
    }

    #[test]
    fn test_kite4() {
        let constraints = build_kite_graph(4);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            to_sols(
                vec![
                    vec![1, 2, 3, 4], // all but 0
                    vec![0, 2, 3, 4], // all but 1
                    vec![0, 1, 3, 4], // all but 2
                    vec![0, 1, 2, 3] // all but 4
                    // but removing 3 also removes 4, so a 4-graph can't be constructed
                ]
            )
        )
    }

    #[test]
    fn test_kite5() {
        let constraints = build_kite_graph(5);
        let nv = constraints.num_vertices.clone();

        let mut solver = SubgraphSolver::new(constraints);

        let sols = collect(&mut solver);

        assert_solutions(
            &nv,
            sols,
            to_sols(vec![vec![0, 1, 2, 3, 4]])
        )
    }
}
