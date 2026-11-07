use std::iter::Sum;

use super::Expression;

///
/// Sums expressions.
///
/// Uses a balanced tree of summations to avoid
/// creating long chains of reference-counted
/// expression nodes. Chains can cause stack
/// overflows when there's a cascade of [`Drop`]
/// calls.
///
/// It's also more accurate, since each term
/// passes through about `log2(n)` additions instead
/// of up to `n`, so rounding errors pile up less.
///
/// TODO: Revisit if we move to a
/// struct-of-arrays architecture, where freeing
/// doesn't nest.
///
impl Sum for Expression {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let terms: Box<[_]> = iter.collect();

        balanced_sum(&terms)
    }
}

fn balanced_sum(terms: &[Expression]) -> Expression {
    match terms {
        [] => 0.0.into(),
        [term] => term.clone(),
        _ => {
            let (left, right) = terms.split_at(terms.len() / 2);

            balanced_sum(left) + balanced_sum(right)
        }
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
    fn long_sum() {
        let s: Expression = (0..100_000).map(|_| Expression::from(1.0)).sum();

        assert_eq!(s.data(), 100_000.0, "long sums stay shallow enough to drop");
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
