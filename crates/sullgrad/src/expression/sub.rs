use std::ops::Sub;

use super::Expression;

impl<R: Into<Self>> Sub<R> for Expression {
    type Output = Self;

    fn sub(self, rhs: R) -> Self {
        self + -rhs.into()
    }
}

impl<R: Into<Expression>> Sub<R> for &Expression {
    type Output = Expression;

    fn sub(self, rhs: R) -> Expression {
        self.clone() - rhs
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
