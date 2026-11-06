use std::ops::Sub;

use super::Expression;

impl Sub<Self> for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{assert_approx_eq, backward_gradients, numerical_gradients};

    #[test]
    fn sub() {
        let x = Expression::from(5.0);
        let y = Expression::from(3.0);
        let s = x - y;

        assert_eq!(s.data(), 2.0, "sub result data");
        assert_eq!(
            s.left().map(Expression::data),
            Some(5.0),
            "sub has left operand"
        );
        assert_eq!(
            s.right().map(Expression::data),
            Some(-3.0),
            "sub has right operand"
        );
        assert_eq!(s.symbol(), "+", "sub has operator");
    }

    #[test]
    fn gradient() {
        let f = |[x, y]: [Expression; 2]| x - y;

        assert_approx_eq(
            backward_gradients(f, [1.5, -2.0]),
            numerical_gradients(f, [1.5, -2.0]),
            "sub gradient matches its numerical slope",
        );
    }
}
