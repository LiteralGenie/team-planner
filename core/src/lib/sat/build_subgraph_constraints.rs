use std::{ collections::HashSet };

use logicng::formulas::{ EncodedFormula, FormulaFactory, Variable };

use crate::console::log;

pub struct SubgraphConstraints {
    pub factory: FormulaFactory,
    pub formula: EncodedFormula,
    pub num_constraints: usize,
    pub num_vertices: i32,
    pub subgraph_size: i32,
}

pub fn build_subgraph_contraints(
    num_vertices: i32,
    subgraph_size: i32,
    edges: HashSet<(i32, i32)>
) -> SubgraphConstraints {
    let f = FormulaFactory::new();

    let n = num_vertices as usize;
    let iter_n = Vec::from_iter(0..n);

    let k = subgraph_size as usize;
    let iter_k = Vec::from_iter(0..k);

    // --- Variables ---

    // Vertices (true if in subgraph)
    let vs: Vec<EncodedFormula> = iter_n
        .iter()
        .map(|i| f.variable(format!("v{}", i).as_str()))
        .collect();

    // Edges
    let es: Vec<Vec<EncodedFormula>> = iter_n
        .iter()
        .map(|i|
            iter_n
                .iter()
                .map(|j|
                    f.variable(format!("e_{}_{}", i, j).as_str())
                )
                .collect()
        )
        .collect();

    // Decisions (vertex picked per timestep)
    let ds: Vec<Vec<EncodedFormula>> = iter_k
        .iter()
        .map(|t|
            iter_n
                .iter()
                .map(|j|
                    f.variable(format!("d_{}_{}", t, j).as_str())
                )
                .collect()
        )
        .collect();

    // --- Constraints ---

    let mut constraints = Vec::<EncodedFormula>::new();

    // Subgraph is non-empty
    constraints.push(f.or(&vs));

    // Edges are undirected
    for i in iter_n.clone() {
        for j in iter_n.clone() {
            if i == j {
                continue;
            }

            constraints.push(f.implication(es[i][j], es[j][i]));
        }
    }

    // Include vertices from decisions
    for t in iter_k.clone() {
        for i in iter_n.clone() {
            constraints.push(
                f.implication(ds[t][i].clone(), vs[i].clone())
            );
        }
    }

    // Include *only* vertices from decisions
    for i in iter_n.clone() {
        let any_timestep = f.or(
            &Vec::from_iter(
                iter_k
                    .clone()
                    .into_iter()
                    .map(|t| ds[t][i].clone())
            )
        );
        constraints.push(f.implication(vs[i].clone(), any_timestep));
    }

    // One decision per timestep
    // @todo: verify this exactly-one stuff works
    for t in iter_k.clone() {
        let vars = Vec::<Variable>::from_iter(
            // ds[t][0].variables(&f).iter()
            iter_n
                .iter()
                .map(|i| f.var(format!("d_{}_{}", t, i).as_str()))
        );
        let decisions_for_timestep = f.exo(vars);
        constraints.push(decisions_for_timestep);
    }

    // For t >= 1, the node picked at each timestep
    // must share an edge with one of the previous nodes
    for t1 in iter_k.clone().into_iter().skip(1) {
        for i in iter_n.clone().into_iter() {
            let mut is_connected_to_prev_edge =
                Vec::<EncodedFormula>::new();

            for t0 in 0..t1 {
                // "If node j was picked at time t0, it shares an edge with node i"
                for j in iter_n.clone().into_iter() {
                    is_connected_to_prev_edge.push(
                        f.and(&vec![ds[t0][j], es[i][j]])
                    );
                }
            }

            // """
            // node i being picked at time t1, implies one of the following
            //   node 0 was picked at time (t1 - 1) and shares an edge with node i
            //   node 1 was picked at time (t1 - 1) and shares an edge with node i
            //   ...
            //   node n was picked at time (t1 - 1) and shares an edge with node i
            //   node 0 was picked at time (t1 - 2) and shares an edge with node i
            //   node 1 was picked at time (t1 - 2) and shares an edge with node i
            //   ...
            //   node n was picked at time 0 and shares an edge with node i
            // """
            constraints.push(
                f.implication(
                    ds[t1][i],
                    f.or(&is_connected_to_prev_edge)
                )
            );
        }
    }

    // The nodes picked at t0 and t1 can't be equal (assuming t0 != t1)
    for i in iter_n.clone().into_iter() {
        for t0 in iter_k.clone().into_iter() {
            for t1 in iter_k.clone().into_iter() {
                if t0 == t1 {
                    continue;
                }

                constraints.push(
                    f.not(f.and(&[ds[t0][i], ds[t1][i]]))
                );
            }
        }
    }

    // --- Init edge connections ---
    for i in iter_n.clone().into_iter() {
        for j in iter_n.clone().into_iter() {
            let constraint = es[i][j];
            let edge = (i as i32, j as i32);
            let edge_reversed = (j as i32, i as i32);

            if i == j {
                // Vertices have edge to self
                constraints.push(constraint);
            } else if
                edges.contains(&edge) | edges.contains(&edge_reversed)
            {
                constraints.push(constraint);
            } else {
                constraints.push(f.not(constraint));
            }
        }
    }

    let num_constraints = constraints.len();
    let all_constraints = f.and(&constraints);

    SubgraphConstraints {
        factory: f,
        formula: all_constraints,
        num_constraints,
        num_vertices,
        subgraph_size,
    }
}
