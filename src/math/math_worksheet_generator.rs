use crate::math::expression::Expression;
use rand::prelude::*;

pub fn generate_questions() -> Vec<Expression> {
    let mut rng = rand::rng();
    let mut expressions: Vec<Expression> = Vec::new();

    for _ in (0..5).rev() {
        let term1d1 = rng.random_range(3..10);
        let term1d2 = rng.random_range(1..9);
        let term2d1 = rng.random_range(2..term1d1);
        let term2d2 = rng.random_range(term1d2 + 1..10);

        expressions.push(Expression {
            term1: term1d1 * 10 + term1d2,
            term2: term2d1 * 10 + term2d2,
            operation: i32::checked_add,
        });
    }

    expressions
}
