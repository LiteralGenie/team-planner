use super::{ Clause, BooleanExpression, BooleanExpressionMode };

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
