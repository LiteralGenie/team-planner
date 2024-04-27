use super::{ Clause, Literal };

#[derive(Clone, Debug)]
pub enum BooleanExpressionMode {
    And,
    Or,
}

#[derive(Clone, Debug)]
pub struct BooleanExpression {
    pub mode: BooleanExpressionMode,
    pub literals: Vec<Literal>,
    pub expressions: Vec<BooleanExpression>,
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

        expr.expressions = expr.expressions
            .iter()
            .map(|sub| sub.negate())
            .collect();

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

#[cfg(test)]
mod tests {
    use crate::lib::sat::*;
    use self::utils::assert_cnf;

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
        assert_cnf(expr, vec![vec![-1, 2]]);

        let expr = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1, l2],
            expressions: vec![],
        };
        assert_cnf(expr, vec![vec![-1], vec![2]]);
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
        assert_cnf(or_or, vec![vec![1, 2, 3, 4]]);

        //   (a + b + (c * d))
        // = ((a + b + c) * (a + b + d))
        let or_and = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            literals: vec![l1, l2],
            expressions: vec![BooleanExpression {
                mode: BooleanExpressionMode::And,
                literals: vec![l3, l4],
                expressions: vec![],
            }],
        };
        assert_cnf(or_and, vec![vec![1, 2, 3], vec![1, 2, 4]]);

        // a * b * (c + d)
        let and_or = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1, l2],
            expressions: vec![BooleanExpression {
                mode: BooleanExpressionMode::Or,
                literals: vec![l3, l4],
                expressions: vec![],
            }],
        };
        assert_cnf(and_or, vec![vec![1], vec![2], vec![3, 4]]);

        //   a * b * (c * d)
        // = a * b * c * d
        let and_or = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1, l2],
            expressions: vec![BooleanExpression {
                mode: BooleanExpressionMode::And,
                literals: vec![l3, l4],
                expressions: vec![],
            }],
        };
        assert_cnf(and_or, vec![vec![1], vec![2], vec![3], vec![4]])
    }

    #[test]
    fn expr_or_or_or() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };
        let l5 = Literal { variable: 5, negate: false };

        //   (a + (b + c) + (d + e))
        // = (a + b + c + d + e)
        let expr = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            literals: vec![l1],
            expressions: vec![
                BooleanExpression {
                    mode: BooleanExpressionMode::Or,
                    literals: vec![l2, l3],
                    expressions: vec![],
                },
                BooleanExpression {
                    mode: BooleanExpressionMode::Or,
                    literals: vec![l4, l5],
                    expressions: vec![],
                }
            ],
        };
        assert_cnf(expr, vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn expr_or_or_and() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };
        let l5 = Literal { variable: 5, negate: false };

        //   (a + (b + c) + (d * e))
        // = (a + b + c + d) * (a + b + c + e)
        let expr = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            literals: vec![l1],
            expressions: vec![
                BooleanExpression {
                    mode: BooleanExpressionMode::Or,
                    literals: vec![l2, l3],
                    expressions: vec![],
                },
                BooleanExpression {
                    mode: BooleanExpressionMode::And,
                    literals: vec![l4, l5],
                    expressions: vec![],
                }
            ],
        };
        assert_cnf(expr, vec![vec![1, 2, 3, 4], vec![1, 2, 3, 5]]);
    }

    #[test]
    fn expr_or_and_and() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };
        let l5 = Literal { variable: 5, negate: false };

        //   a + (b * c) + (d * e)
        // = (a + b) * (a + c) + (d * e)
        // = (d + (a + b) * (a + c)) * (e + (a + b) * (a + c))
        // = (dab) * (dac) * (eab) * (eac)
        let expr = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            literals: vec![l1],
            expressions: vec![
                BooleanExpression {
                    mode: BooleanExpressionMode::And,
                    literals: vec![l2, l3],
                    expressions: vec![],
                },
                BooleanExpression {
                    mode: BooleanExpressionMode::And,
                    literals: vec![l4, l5],
                    expressions: vec![],
                }
            ],
        };
        assert_cnf(expr, vec![vec![4, 1, 2], vec![4, 1, 3], vec![5, 1, 2], vec![5, 1, 3]]);
    }

    #[test]
    fn expr_and_or_or() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };
        let l5 = Literal { variable: 5, negate: false };

        // a * (b + c) * (d + e)
        let expr = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1],
            expressions: vec![
                BooleanExpression {
                    mode: BooleanExpressionMode::Or,
                    literals: vec![l2, l3],
                    expressions: vec![],
                },
                BooleanExpression {
                    mode: BooleanExpressionMode::Or,
                    literals: vec![l4, l5],
                    expressions: vec![],
                }
            ],
        };

        assert_cnf(expr, vec![vec![1], vec![2, 3], vec![4, 5]]);
    }

    #[test]
    fn expr_and_or_and() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };
        let l5 = Literal { variable: 5, negate: false };

        // a * (b + c) * (d * e)
        let expr = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1],
            expressions: vec![
                BooleanExpression {
                    mode: BooleanExpressionMode::Or,
                    literals: vec![l2, l3],
                    expressions: vec![],
                },
                BooleanExpression {
                    mode: BooleanExpressionMode::And,
                    literals: vec![l4, l5],
                    expressions: vec![],
                }
            ],
        };

        assert_cnf(expr, vec![vec![1], vec![2, 3], vec![4], vec![5]]);
    }
    #[test]
    fn expr_and_and_and() {
        let l1 = Literal { variable: 1, negate: false };
        let l2 = Literal { variable: 2, negate: false };
        let l3 = Literal { variable: 3, negate: false };
        let l4 = Literal { variable: 4, negate: false };
        let l5 = Literal { variable: 5, negate: false };

        // a * (b * c) * (d * e)
        let expr = BooleanExpression {
            mode: BooleanExpressionMode::And,
            literals: vec![l1],
            expressions: vec![
                BooleanExpression {
                    mode: BooleanExpressionMode::And,
                    literals: vec![l2, l3],
                    expressions: vec![],
                },
                BooleanExpression {
                    mode: BooleanExpressionMode::And,
                    literals: vec![l4, l5],
                    expressions: vec![],
                }
            ],
        };

        assert_cnf(expr, vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
    }
}
