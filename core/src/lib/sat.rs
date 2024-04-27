#[derive(Clone, Copy)]
pub struct Literal {
    pub variable: i32,
    pub negate: bool,
}

impl Literal {
    pub fn dump(&self) -> i32 {
        match self.negate {
            true => -self.variable,
            false => self.variable,
        }
    }
}

pub struct Clause {
    pub literals: Vec<Literal>,
}

impl Clause {
    pub fn dump(&self) -> Vec<i32> {
        let lits = self.literals.iter().map(|l| l.dump());
        Vec::from_iter(lits)
    }

    pub fn add(&self, other: &Self) -> Clause {
        let mut literals = self.literals.clone();
        literals.extend(other.literals.iter());

        Clause {
            literals,
        }
    }
}

pub struct CnfBuilder {
    pub num_variables: i32,
    pub clauses: Vec<Clause>,
}

impl Default for CnfBuilder {
    fn default() -> Self {
        Self { num_variables: 0, clauses: Vec::new() }
    }
}

impl CnfBuilder {
    pub fn add_variable(&mut self) -> i32 {
        self.num_variables += 1;
        self.num_variables
    }

    pub fn dump(&self) -> Vec<Vec<i32>> {
        Vec::from_iter(
            self.clauses
                .as_slice()
                .iter()
                .map(|c| c.dump())
        )
    }
}

#[derive(Clone)]
pub enum BooleanExpressionMode {
    And,
    Or,
}

#[derive(Clone)]
pub struct BooleanExpression {
    mode: BooleanExpressionMode,
    literals: Vec<Literal>,
    expressions: Vec<BooleanExpression>,
}

impl Default for BooleanExpression {
    fn default() -> Self {
        Self {
            mode: BooleanExpressionMode::Or,
            literals: Vec::new(),
            expressions: Vec::new(),
        }
    }
}

impl BooleanExpression {
    pub fn dump(&self) -> Vec<Clause> {
        let mut clauses = Vec::new();

        match self.mode {
            BooleanExpressionMode::And => {
                // Each sub expression becomes a CNF
                // And since CNF means a set of AND'd clauses
                // AND'ing each resulting CNF yields a new CNF
                //   for example, if this expression has two subexpressions e1=(a+b) and e2=((c+d) * (e+f))
                //     BinaryExpression(
                //          BinaryExpression(a + b)
                //        * BinaryExpression(
                //              BinaryExpression(c + d)
                //            * BinaryExpression(e + f)
                //          )
                //      )
                //   recursively calling e1.dump() and e2.dump() gives us two CNFs in the form of clause lists
                //      e1.dump() = [Clause(a+b)]
                //      e2.dump() = [Clause(c+d), Clause(e+f)]
                //      (each term in the list is implicitly AND'd)
                //   and we can AND these lists together
                //      result = e1.dump() + e2.dump()
                //             = [Clause(a+b), Clause(c+d), Clause(e+f)]
                clauses.extend(self.expressions.iter().flat_map(|expr| expr.dump()));

                // We can treat literals as an AND expression with one term and do the same thing
                clauses.extend(self.literals.iter().map(|l| Clause { literals: vec![l.clone()] }));
            }

            BooleanExpressionMode::Or => {
                // Like the AND-case above we want to normalize things by
                //   converting the sub expressions into CNF form (lists of clauses)
                //
                // But since we're in OR-mode, we only have something like this
                //    (a+b)(c+d) + (e+f)(g+h)
                //    ^^^^^^^^^^   ^^^^^^^^^^
                //    sub-expr 1   sub-expr 2
                //
                // To get everything into a single CNF, we can
                //   distribute each clause in the second expression like this
                //
                //     (a+b)(c+d) + (e+f)(g+h)                  [0]
                //   = ((a+b) + (e+f)(g+h)) ((c+d) + (e+f)(g+h))
                //
                // And for each half, doing another distribution yields
                //     (a+b) + (e+f)(g+h)                       [1]
                //   = (a+b + e+f)(a+b + g+h)
                //
                // Since this is in CNF form, and since each
                //   of the halves (quarters) we distributed on are AND'd,
                //   the final result is also in CNF form
                //
                //     (a+b)(c+d) + (e+f)(g+h)
                //   = (a+b+e+f) (a+b+g+h) (c+d+e+f) (c+d+g+h)
                //
                // Which is really just saying
                //   xy + uv = (x + u) (x + v) (y + u) (y + v)
                //
                // or "take the cartesian product of clauses from each expression and OR the literals inside each pair of clauses"
                let cnfs = self.expressions.iter().map(|expr| expr.dump());

                let cnfs = cnfs.chain(
                    self.literals
                        .iter()
                        .map(|literal| { vec![Clause { literals: vec![*literal] }] })
                );

                let result = cnfs.reduce(|acc, cnf| distribute(acc, cnf));

                if let Some(cs) = result {
                    clauses.extend(cs);
                }
            }
        }
        clauses
    }

    pub fn negate(&self) -> BooleanExpression {
        let mut expr = self.clone();

        for lit in expr.literals.iter_mut() {
            lit.negate = !lit.negate;
        }

        expr.mode = match expr.mode {
            BooleanExpressionMode::Or => BooleanExpressionMode::And,
            BooleanExpressionMode::And => BooleanExpressionMode::Or,
        };

        expr
    }
}

fn distribute(left: Vec<Clause>, right: Vec<Clause>) -> Vec<Clause> {
    let mut result = Vec::<Clause>::new();

    for c1 in left {
        for c2 in right.iter() {
            result.push(c1.add(c2));
        }
    }

    result
}

// x1 | x2
pub fn Or(x1: BooleanExpression, x2: BooleanExpression) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![x1, x2],
        ..Default::default()
    }
}

// x1 & x2
pub fn And(x1: BooleanExpression, x2: BooleanExpression) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![x1, x2],
        ..Default::default()
    }
}

pub fn Neg(mut x1: BooleanExpression) -> BooleanExpression {
    x1.negate();
    x1
}

// x1 => x2
pub fn Implies(x1: BooleanExpression, x2: BooleanExpression) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![Neg(x1), x2],
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::lib::sat::*;

    fn vec_vec_to_vec_hash(cnf: Vec<Vec<i32>>) -> Vec<HashSet<i32>> {
        cnf.into_iter()
            .map(|clause| {
                let n = clause.len();
                let hashed = HashSet::from_iter(clause.into_iter());

                // No dupes should be removed, each variable should appear at most once
                assert_eq!(hashed.len(), n);

                hashed
            })
            .collect()
    }

    fn assert_expression(expr: BooleanExpression, expected_value: Vec<Vec<i32>>) {
        let actual_value: Vec<Vec<i32>> = expr
            .dump()
            .iter()
            .map(|clause| clause.dump())
            .collect();

        // Remove ordering
        let expected_value: Vec<HashSet<i32>> = vec_vec_to_vec_hash(expected_value);
        let actual_value: Vec<HashSet<i32>> = vec_vec_to_vec_hash(actual_value);

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn literal_dump() {
        let t = Literal { variable: 1, negate: true };
        assert_eq!(t.dump(), -1);

        let f = Literal { variable: 1, negate: false };
        assert_eq!(f.dump(), 1);
    }

    #[test]
    fn clause_dump() {
        let x1 = Literal { variable: 1, negate: true };
        let x2 = Literal { variable: 2, negate: false };

        let clause = Clause { literals: vec![x1, x2] };
        assert_eq!(clause.dump(), vec![-1, 2]);
    }

    #[test]
    fn expr_dump() {
        let l1 = Literal { variable: 1, negate: true };
        let l2 = Literal { variable: 2, negate: false };

        let expr = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            literals: vec![l1, l2],
            expressions: vec![],
        };
        assert_expression(expr, vec![vec![-1, 2]]);

        let expr = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1, l2],
            expressions: vec![],
        };
        assert_expression(expr, vec![vec![-1], vec![2]]);
    }

    #[test]
    fn expr_nested() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };

        //   (a + b + (c + d))
        // = (a + b + c + d)
        let or_or = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            literals: vec![l1, l2],
            expressions: vec![BooleanExpression {
                mode: BooleanExpressionMode::Or,
                literals: vec![l3, l4],
                expressions: vec![],
            }],
        };
        assert_expression(or_or, vec![vec![1, 2, 3, 4]])
    }
}
