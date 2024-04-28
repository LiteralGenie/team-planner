use std::collections::HashSet;

use super::{ build_subgraph_contraints, SubgraphConstraints };

/**
 * a - b
 */
pub fn build_ab_graph(subgraph_size: i32) -> SubgraphConstraints {
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
pub fn build_square_graph(subgraph_size: i32) -> SubgraphConstraints {
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
pub fn build_kite_graph(subgraph_size: i32) -> SubgraphConstraints {
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
