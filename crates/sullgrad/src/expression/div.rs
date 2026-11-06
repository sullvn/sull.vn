use std::ops::Div;

use super::Expression;

impl Div<Self> for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.pow((-1.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{assert_approx_eq, backward_gradients, numerical_gradients};

    #[test]
    fn div() {
        let x = Expression::from(6.0);
        let y = Expression::from(2.0);
        let d = x / y;

        assert_eq!(d.data(), 3.0, "div result data");
        assert_eq!(
            d.left().map(Expression::data),
            Some(6.0),
            "div has left operand"
        );
        assert_eq!(
            d.right().map(Expression::data),
            Some(0.5),
            "div has right operand"
        );
        assert_eq!(d.symbol(), "*", "div has operator");
    }

    #[test]
    fn gradient() {
        let f = |[x, y]: [Expression; 2]| x / y;

        assert_approx_eq(
            backward_gradients(f, [1.5, -2.0]),
            numerical_gradients(f, [1.5, -2.0]),
            "div gradient matches its numerical slope",
        );
    }
}
