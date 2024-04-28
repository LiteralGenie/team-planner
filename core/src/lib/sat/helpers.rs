use super::{ BooleanExpression, BooleanExpressionMode };

pub fn Or(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![x1.clone(), x2.clone()],
        ..Default::default()
    }
}

pub fn OrAll(xs: &Vec<BooleanExpression>) -> BooleanExpression {
    xs.clone()
        .into_iter()
        .reduce(|acc, x| Or(&acc, &x))
        .unwrap_or_else(|| BooleanExpression { ..Default::default() })
        .clone()
}

pub fn And(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::And,
        expressions: vec![x1.clone(), x2.clone()],
        ..Default::default()
    }
}

pub fn Not(x1: &BooleanExpression) -> BooleanExpression {
    x1.negate()
}

pub fn Implies(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    // a => b  <=>  ~a | b
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![Not(x1), x2.clone()],
        ..Default::default()
    }
}

pub fn Xor(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    // a xor b <=> (a + b) * ~(a * b)
    //         <=> "a or b is true but not both"
    let at_least_one = Or(x1, x2);
    let at_most_one = Not(&And(&x1, &x2));

    And(&at_least_one, &at_most_one)
}

#[cfg(test)]
mod tests {
    use crate::lib::sat::*;

    use self::utils::assert_cnf;

    fn lit_expr(variable: i32, negate: bool) -> BooleanExpression {
        BooleanExpression {
            literals: vec![Literal { variable, negate }],
            ..Default::default()
        }
    }

    #[test]
    fn or() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);

        let expr = Or(&e1, &e2);
        assert_cnf(expr, vec![vec![1, -2]])
    }

    #[test]
    fn and() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);

        let expr = And(&e1, &e2);
        assert_cnf(expr, vec![vec![1], vec![-2]])
    }

    #[test]
    fn not_or() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);
        let e3 = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            expressions: vec![e1, e2],
            ..Default::default()
        };

        //   ~(a + b)
        // = ~a * ~b
        let expr = Not(&e3);
        assert_cnf(expr, vec![vec![-1], vec![2]])
    }

    #[test]
    fn not_and() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);
        let e3 = BooleanExpression {
            mode: BooleanExpressionMode::And,
            expressions: vec![e1, e2],
            ..Default::default()
        };

        //   ~(a * b)
        // = ~a + ~b
        let expr = Not(&e3);
        assert_cnf(expr, vec![vec![-1, 2]])
    }

    #[test]
    fn implies() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, false);

        //   a => b
        // = ~a + b
        let expr = Implies(&e1, &e2);
        assert_cnf(expr, vec![vec![-1, 2]])
    }

    #[test]
    fn xor() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, false);

        //   a xor b
        // = (a + b) * ~(a * b)
        // = (a + b) * (~a + ~b)
        let expr = Xor(&e1, &e2);
        assert_cnf(expr, vec![vec![1, 2], vec![-1, -2]])
    }
}
