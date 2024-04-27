use std::{ collections::HashSet };

use super::{
    And,
    BooleanExpression,
    CnfBuilder,
    Implies,
    Not,
    OrAll,
    Xor,
};

pub fn build_subgraph_contraints(
    num_vertices: i32,
    subgraph_size: i32,
    edges: HashSet<(i32, i32)>
) -> Vec<BooleanExpression> {
    let n = num_vertices as usize;
    let iter_n = Vec::from_iter(0..n);

    let k = subgraph_size as usize;
    let iter_k = Vec::from_iter(0..k);

    let mut builder = CnfBuilder { ..Default::default() };

    // --- Init variables ---

    // Vertices (true if in subgraph)
    let vs: Vec<BooleanExpression> = iter_n
        .iter()
        .map(|i|
            BooleanExpression::from_variable(builder.add_variable())
        )
        .collect();

    // Edges
    let es: Vec<Vec<BooleanExpression>> = iter_n
        .iter()
        .map(|i|
            iter_n
                .iter()
                .map(|j|
                    BooleanExpression::from_variable(
                        builder.add_variable()
                    )
                )
                .collect()
        )
        .collect();

    // Decisions (vertex picked per timestep)
    let ds: Vec<Vec<BooleanExpression>> = iter_n
        .iter()
        .map(|i|
            iter_n
                .iter()
                .map(|j|
                    BooleanExpression::from_variable(
                        builder.add_variable()
                    )
                )
                .collect()
        )
        .collect();

    // --- Define constraints ---

    let mut constraints = Vec::<BooleanExpression>::new();

    // Subgraph is non-empty
    constraints.push(OrAll(&vs));

    // Edges are undirected
    for i in iter_n.clone() {
        for j in iter_n.clone() {
            constraints.push(
                Implies(&es[i][j].clone(), &es[j][j].clone())
            );
        }
    }

    // Include vertices from decisions
    for t in iter_k.clone() {
        for i in iter_n.clone() {
            constraints.push(
                Implies(&ds[t][i].clone(), &vs[i].clone())
            );
        }
    }

    // Include *only* vertices from decisions
    for i in iter_n.clone() {
        let any_timestep = OrAll(
            &iter_k
                .clone()
                .into_iter()
                .map(|t| ds[t][i].clone())
                .collect()
        );
        constraints.push(Implies(&vs[i].clone(), &any_timestep));
    }

    // One decision per timestep
    for t in iter_k.clone() {
        let decisions_for_timestep = ds[t]
            .clone()
            .into_iter()
            .reduce(|acc, d| Xor(&acc, &d.clone()))
            .unwrap();
        constraints.push(decisions_for_timestep);
    }

    // For t > 1, the node picked at each timestep
    // must share an edge with one of the previous nodes
    for t1 in iter_k.clone().into_iter().skip(1) {
        for i in iter_n.clone().into_iter() {
            let mut is_connected_to_prev_edge =
                Vec::<BooleanExpression>::new();

            for t0 in 0..t1 {
                // "If node j was picked at time t0, it shares an edge with node i"
                for j in iter_n.clone().into_iter() {
                    is_connected_to_prev_edge.push(
                        And(&ds[t0][j], &es[i][j])
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
                Implies(
                    &ds[t1][i],
                    &OrAll(&is_connected_to_prev_edge)
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

                constraints.push(Not(&And(&ds[t0][i], &ds[t1][i])));
            }
        }
    }

    // --- Init edge connections ---
    for i in iter_n.clone().into_iter() {
        for j in iter_n.clone().into_iter() {
            let constraint = es[i][j].clone();
            let edge = (i as i32, j as i32);

            if i == j {
                // Vertices have edge to self
                constraints.push(constraint);
            } else if edges.contains(&edge) {
                constraints.push(constraint);
            } else {
                constraints.push(Not(&constraint));
            }
        }
    }

    constraints
}
