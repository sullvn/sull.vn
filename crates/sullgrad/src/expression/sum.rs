use std::iter::Sum;
use std::ops::Add;

use super::Expression;

impl Sum for Expression {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(0.0.into(), Add::add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{assert_approx_eq, backward_gradients, numerical_gradients};

    #[test]
    fn sum() {
        let s: Expression = [1.0, 2.0, 3.0].map(Expression::from).into_iter().sum();

        assert_eq!(s.data(), 6.0, "sum result data");
        assert_eq!(s.symbol(), "+", "sum has operator");
    }

    #[test]
    fn empty_sum() {
        let s: Expression = std::iter::empty().sum();

        assert_eq!(s.data(), 0.0, "empty sum is zero");
    }

    #[test]
    fn gradient() {
        let f = |xs: [Expression; 3]| xs.into_iter().sum();

        assert_approx_eq(
            backward_gradients(f, [1.5, -2.0, 0.5]),
            numerical_gradients(f, [1.5, -2.0, 0.5]),
            "sum gradient matches its numerical slope",
        );
    }
}
