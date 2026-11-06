use std::ops::Neg;

use super::Expression;

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self {
        self * (-1.0).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{assert_approx_eq, backward_gradients, numerical_gradients};

    #[test]
    fn neg() {
        let x = Expression::from(2.0);
        let n = -x;

        assert_eq!(n.data(), -2.0, "neg result data");
        assert_eq!(
            n.left().map(Expression::data),
            Some(2.0),
            "neg has left operand"
        );
        assert_eq!(
            n.right().map(Expression::data),
            Some(-1.0),
            "neg has right operand"
        );
        assert_eq!(n.symbol(), "*", "neg has operator");
    }

    #[test]
    fn gradient() {
        let f = |[x]: [Expression; 1]| -x;

        assert_approx_eq(
            backward_gradients(f, [1.5]),
            numerical_gradients(f, [1.5]),
            "neg gradient matches its numerical slope",
        );
    }
}
