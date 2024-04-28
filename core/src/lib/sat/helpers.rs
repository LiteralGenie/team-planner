use super::{ BooleanExpression, BooleanExpressionMode };

pub fn or(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![x1.clone(), x2.clone()],
        ..Default::default()
    }
}

pub fn or_all(xs: &Vec<BooleanExpression>) -> BooleanExpression {
    xs.clone()
        .into_iter()
        .reduce(|acc, x| or(&acc, &x))
        .unwrap_or_else(|| BooleanExpression { ..Default::default() })
        .clone()
}

pub fn and(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    BooleanExpression {
        mode: BooleanExpressionMode::And,
        expressions: vec![x1.clone(), x2.clone()],
        ..Default::default()
    }
}

pub fn and_all(xs: &Vec<BooleanExpression>) -> BooleanExpression {
    xs.clone()
        .into_iter()
        .reduce(|acc, x| and(&acc, &x))
        .unwrap_or_else(|| BooleanExpression { ..Default::default() })
        .clone()
}

pub fn not(x1: &BooleanExpression) -> BooleanExpression {
    x1.negate()
}

pub fn implies(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    // a => b  <=>  ~a | b
    BooleanExpression {
        mode: BooleanExpressionMode::Or,
        expressions: vec![not(x1), x2.clone()],
        ..Default::default()
    }
}

pub fn exactly_one(xs: Vec<BooleanExpression>) -> BooleanExpression {
    let at_least_one = or_all(
        &Vec::from_iter(xs.iter().map(|x| x.clone()))
    );

    let mut at_most_one = Vec::<BooleanExpression>::new();
    for (i, x1) in xs.iter().enumerate() {
        for (j, x2) in xs.iter().enumerate() {
            if i == j {
                continue;
            }

            at_most_one.push(or(&not(&x1), &not(&x2)));
        }
    }

    let at_most_one = and_all(&at_most_one);

    and(&at_least_one, &at_most_one)
}

pub fn xor(
    x1: &BooleanExpression,
    x2: &BooleanExpression
) -> BooleanExpression {
    // a xor b <=> (a + b) * ~(a * b)
    //         <=> "a or b is true but not both"
    let at_least_one = or(x1, x2);
    let at_most_one = not(&and(&x1, &x2));

    and(&at_least_one, &at_most_one)
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
    fn test_or() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);

        let expr = or(&e1, &e2);
        assert_cnf(expr, vec![vec![1, -2]])
    }

    #[test]
    fn test_and() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);

        let expr = and(&e1, &e2);
        assert_cnf(expr, vec![vec![1], vec![-2]])
    }

    #[test]
    fn test_not_or() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);
        let e3 = BooleanExpression {
            mode: BooleanExpressionMode::Or,
            expressions: vec![e1, e2],
            ..Default::default()
        };

        //   ~(a + b)
        // = ~a * ~b
        let expr = not(&e3);
        assert_cnf(expr, vec![vec![-1], vec![2]])
    }

    #[test]
    fn test_not_and() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, true);
        let e3 = BooleanExpression {
            mode: BooleanExpressionMode::And,
            expressions: vec![e1, e2],
            ..Default::default()
        };

        //   ~(a * b)
        // = ~a + ~b
        let expr = not(&e3);
        assert_cnf(expr, vec![vec![-1, 2]])
    }

    #[test]
    fn test_implies() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, false);

        //   a => b
        // = ~a + b
        let expr = implies(&e1, &e2);
        assert_cnf(expr, vec![vec![-1, 2]])
    }

    #[test]
    fn test_xor() {
        let e1 = lit_expr(1, false);
        let e2 = lit_expr(2, false);

        //   a xor b
        // = (a + b) * ~(a * b)
        // = (a + b) * (~a + ~b)
        let expr = xor(&e1, &e2);
        assert_cnf(expr, vec![vec![1, 2], vec![-1, -2]])
    }
}
