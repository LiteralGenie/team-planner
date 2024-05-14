use std::{ collections::{ HashMap, HashSet } };

use logicng::formulas::{ EncodedFormula, FormulaFactory };
use itertools::Itertools;

use crate::console::log;

use super::{
    build_subgraph_contraints,
    HashIntSet,
    SubgraphConstraints,
};

pub fn build_champion_constraints(
    graph_size: u8,
    subgraph_size: u8,
    slot_options: &Vec<Vec<u8>>,
    champion_traits: &HashMap<u8, Vec<String>>
) -> SubgraphConstraints {
    // Assign edges to champions that share traits
    let mut grouped_by_trait = HashMap::<String, Vec<u8>>::new();
    for (champion, traits) in champion_traits.iter() {
        for t in traits {
            let cs = grouped_by_trait
                .entry(t.clone())
                .or_insert(vec![]);
            cs.push(*champion);
        }
    }

    let edges = HashSet::<(i32, i32)>::from_iter(
        grouped_by_trait.values().flat_map(|cs| {
            // Calculate product minus dupes
            let mut product = Vec::<(i32, i32)>::new();

            for (i, c1) in cs.iter().enumerate() {
                for c2 in cs.iter().skip(i + 1) {
                    product.push((*c1 as i32, *c2 as i32));
                }
            }

            return product;
        })
    );

    let mut constraints = build_subgraph_contraints(
        graph_size as i32,
        subgraph_size as i32,
        edges
    );

    let mut new_constraints = build_slot_constraints(
        slot_options,
        &constraints
    );

    // Merge subgraph and slot constraints
    constraints.num_constraints += new_constraints.len();

    new_constraints.push(constraints.formula);
    constraints.formula = constraints.factory.and(&new_constraints);

    constraints
}

#[derive(Debug)]
struct ChampionSubset {
    champions: HashSet<u8>,
    parent_sets: Vec<usize>,
}

impl ChampionSubset {
    fn new() -> Self {
        Self {
            champions: HashSet::new(),
            parent_sets: vec![],
        }
    }
}

// @TODO: really need to decouple this from SubgraphConstraints / FormulaFactory for testing
fn build_slot_constraints(
    slot_options: &Vec<Vec<u8>>,
    subgraph_constraints: &SubgraphConstraints
) -> Vec<EncodedFormula> {
    log!("slot options {:?}", slot_options);

    let mut disjoint_subsets = Vec::<ChampionSubset>::new();

    let slot_options: Vec<HashSet<u8>> = slot_options
        .iter()
        .map(|opts| HashSet::from_iter(opts.clone().into_iter()))
        // @todo: can this optimization be generalized?
        // Slots that contain all champions don't need to be constraints
        .filter(
            |opts|
                opts.len() <
                (subgraph_constraints.num_vertices as usize)
        )
        .collect();

    // Group each element by the parent sets it is common to
    for (idx, options) in slot_options.iter().enumerate() {
        let mut new_set = ChampionSubset::new();
        new_set.champions = HashSet::from_iter(options.clone());
        new_set.parent_sets.push(idx);

        merge_into_disjoint_list(new_set, &mut disjoint_subsets);
    }

    log!(
        "{} disjoint sets {:?}",
        disjoint_subsets.len(),
        disjoint_subsets
    );

    let mut constraints = Vec::<EncodedFormula>::new();
    let f = &subgraph_constraints.factory;

    // Solutions must contain at least one champion from each set of slot options
    for options in slot_options.iter() {
        constraints.push(
            f.or(
                &Vec::from_iter(
                    options
                        .iter()
                        .map(|id| format!("v{}", id))
                        .map(|name| f.variable(name.as_str()))
                )
            )
        );
    }

    // A single champion cannot fill multiple slots
    //
    // For example, if we have two sets of slot options with common options
    //   A = { 1, 2, 3 }
    //   B = { 1, 4, 5 }
    //
    // Then the at-least-one constraints from above are
    //   (1|2|3) & (1|4|5)
    //
    // So (1,3) is a valid solution, but so is (1,_)
    // To force the solution to have another element from one of the slots, we add the additional constraint
    //   1 => (2|3) | (4|5)
    //
    // If we have three sets of slot options
    //   A = { 1, 2, 3 }
    //   B = { 1, 2, 4 }
    //   C = { 1, 2, 5 }
    //
    // Then to prevent (1,_,_) and (2,_,_), we need to add
    //   1      =>  ((2|3) & (2|4)) | ((2|4) & (2|5))
    //   2      =>  ((1|3) & (1|4)) | ((1|4) & (1|5))
    //   1 & 2  =>  3 | 4 | 5
    //
    // @todo finish explanation
    for set in disjoint_subsets {
        let n_parents = set.parent_sets.len();
        if n_parents <= 1 {
            continue;
        }

        let combinations = find_all_products(
            &set.champions,
            n_parents.min(
                (subgraph_constraints.subgraph_size as usize) - 1
            )
        );

        log!(
            "appending {} dedupe constraints involving {} champions from {} sets: {:?}",
            combinations.len(),
            set.champions.len(),
            n_parents,
            set.champions
        );
        for lhs in combinations.iter() {
            let lhs_vars = champion_ids_to_vars(&lhs.0, &f);

            let mut parents_minus_lhs = vec![];
            for idx in set.parent_sets.iter() {
                let mut set = slot_options[*idx].clone();
                for var in lhs.0.iter() {
                    set.remove(var);
                }

                parents_minus_lhs.push(set);
            }

            let num_parents_true = n_parents - lhs.0.len();
            let rhs_vars = Vec::from_iter(
                parents_minus_lhs
                    .iter()
                    .combinations(num_parents_true)
                    .map(|parents| {
                        let parent_formulas = Vec::from_iter(
                            parents
                                .iter()
                                .map(|set|
                                    f.or(
                                        &champion_ids_to_vars(
                                            &set,
                                            &f
                                        )
                                    )
                                )
                        );

                        f.and(&parent_formulas)
                    })
            );

            let c = f.implication(f.and(&lhs_vars), f.or(&rhs_vars));
            constraints.push(c);
        }
    }

    constraints
}

fn champion_ids_to_vars(
    ids: &HashSet<u8>,
    factory: &FormulaFactory
) -> Vec<EncodedFormula> {
    ids.iter()
        .map(|id_champion| format!("v{}", id_champion))
        .map(|name| factory.variable(name.as_str()))
        .collect()
}

/**
 * Find all combinations of the options, up to some length and excluding repeats
 *
 * For example, given the arguments
 *   options = [ "a", "b", "c" ]
 *   length = 3
 *
 * The output will be
 *   {
 *      { "a" }
 *      { "b" }
 *      { "c" }
 *      { "a", "b" }
 *      { "a", "c" }
 *      { "a", "b", "c" }
 *   }
 */
fn find_all_products(
    options: &HashSet<u8>,
    length: usize
) -> HashSet<HashIntSet> {
    // Init with the options mapped to sets
    //   eg [ "a", "b", "c" ] -> { { "a" }, { "b" }, { "c" } }
    let mut iterations = vec![
        HashSet::from_iter(
            options
                .iter()
                .map(|s| HashIntSet::from_vec(&vec![s.clone()]))
        )
    ];

    // Take all the combinations of length N and find all combinations of length N+1
    for _ in 1..length {
        let mut to_add = HashSet::new();

        for set in iterations.last().unwrap().iter() {
            to_add.extend(product(set, &options));
        }

        iterations.push(to_add);
    }

    // Join the iterations into giant hash set
    let mut result = HashSet::new();
    for set in iterations {
        result.extend(set);
    }
    result
}

/**
 * Insert each option into new copies of source
 * eg for the arguments
 *   source  = { "a", "b" }
 *   options = [ "1", "2" ]
 * the output is
 *   {
 *      { "a", "1" },
 *      { "a", "2" },
 *      { "b", "1" },
 *      { "b", "2" }
 *   }
 */
fn product(
    source: &HashIntSet,
    options: &HashSet<u8>
) -> HashSet<HashIntSet> {
    HashSet::from_iter(
        options.iter().map(|opt| {
            let mut s = source.clone();
            s.0.insert(opt.clone());
            s
        })
    )
}

/**
 * If new set looks something like this
 *    { 1, 6, 8 }
 *
 * and the list of existing (disjoint) sets looks like this
 *    [
 *       { 1, 3, 4 },
 *       { 2, 5, 7 },
 *    ]
 *
 * Any common elements like { 1 } are moved from both sets into its own set
 * The remaining elements are also given their own set.
 * ie the list is updated to look like this
 *    [
 *       { 3, 4 },
 *       { 2, 5, 7 },
 *       { 1 },
 *       { 6, 8 },
 *    ]
 *
 * And throughout this process we keep track of the original sets each of these subsets are common to
 * eg assuming our original sets looked something like this
 *    A: { 1, 2, 3, 4, 5, 7 }
 *    B: { 2, 5, 7 }
 *    C: { 1, 6, 8 }
 *
 * Then after the first merge, the disjoint list looks like
 *   [
 *      { 1, 3, 4 }   ->   parents: A
 *      { 2, 5, 7 }   ->   parents: A, B
 *   ]
 *
 * After the next merge, the list looks like this
 *   [
 *      { 3, 4 }      ->   parents: A
 *      { 2, 5, 7 }   ->   parents: A, B
 *      { 1 },        ->   parents: A, C
 *      { 6, 8 },     ->   parents: C
 *   ]
 */
fn merge_into_disjoint_list(
    mut new_set: ChampionSubset,
    disjoint_subsets: &mut Vec<ChampionSubset>
) {
    if disjoint_subsets.len() == 0 {
        disjoint_subsets.push(new_set);
        return;
    }

    let mut to_append = Vec::<ChampionSubset>::new();

    for existing_subset in disjoint_subsets.iter_mut() {
        let mut common = ChampionSubset::new();

        // Elements from new_set that are common to existing_set
        // are common to (single) parent of new_set the sets that generated existing_set
        common.parent_sets.append(&mut new_set.parent_sets.clone());
        common.parent_sets.append(
            &mut existing_subset.parent_sets.clone()
        );

        // Find commons
        for champion in new_set.champions.iter() {
            let in_both =
                existing_subset.champions.contains(champion);

            if in_both {
                common.champions.insert(champion.clone());
            }
        }

        // Remove commons from parent sets
        for champion in common.champions.iter() {
            existing_subset.champions.remove(champion);
            new_set.champions.remove(champion);
        }

        if common.champions.len() > 0 {
            to_append.push(common);
        }
    }

    disjoint_subsets.append(&mut to_append);

    if new_set.champions.len() > 0 {
        disjoint_subsets.push(new_set);
    }

    disjoint_subsets.retain(|s| s.champions.len() > 0);
}
