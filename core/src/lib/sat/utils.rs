use std::{ collections::HashSet, hash::{ Hash, Hasher } };

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

// https://stackoverflow.com/questions/36562419/hashset-as-key-for-other-hashset
#[derive(Clone, Debug)]
pub struct HashStringSet(pub HashSet<String>);

impl PartialEq for HashStringSet {
    fn eq(&self, other: &HashStringSet) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for HashStringSet {}

impl Hash for HashStringSet {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&String> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

impl HashStringSet {
    pub fn from_vec(xs: &Vec<String>) -> Self {
        Self(HashSet::from_iter(xs.clone().into_iter()))
    }
}

#[derive(Clone, Debug)]
pub struct HashIntSet(pub HashSet<u8>);

impl PartialEq for HashIntSet {
    fn eq(&self, other: &HashIntSet) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for HashIntSet {}

impl Hash for HashIntSet {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&u8> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

impl HashIntSet {
    pub fn from_vec(xs: &Vec<u8>) -> Self {
        Self(HashSet::from_iter(xs.clone().into_iter()))
    }
}
