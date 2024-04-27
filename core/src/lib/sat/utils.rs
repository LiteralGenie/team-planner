use super::BooleanExpression;

use std::{ collections::HashSet, hash::{ Hash, Hasher } };

// https://stackoverflow.com/questions/36562419/hashset-as-key-for-other-hashset
#[derive(Debug)]
struct HashedClause(HashSet<i32>);

impl PartialEq for HashedClause {
    fn eq(&self, other: &HashedClause) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for HashedClause {}

impl Hash for HashedClause {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&i32> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

fn vec_vec_to_hash_hash(cnf: Vec<Vec<i32>>) -> HashSet<HashedClause> {
    let result = HashSet::from_iter(
        cnf
            .clone()
            .into_iter()
            .map(|clause| {
                let hashed = HashedClause(HashSet::from_iter(clause.clone().into_iter()));

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
    assert_eq!(result.len(), cnf.len(), "Duplicate clauses removed from {:?}", cnf);

    result
}

pub fn assert_cnf(expr: BooleanExpression, expected_value: Vec<Vec<i32>>) {
    let actual_value: Vec<Vec<i32>> = expr
        .dump()
        .iter()
        .map(|clause| clause.dump())
        .collect();

    // Remove ordering
    let expected_value = vec_vec_to_hash_hash(expected_value);
    let actual_value = vec_vec_to_hash_hash(actual_value);

    assert_eq!(actual_value, expected_value);
}
