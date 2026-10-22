use super::{Expr, Expression};

impl Expression {
    ///
    /// Raises e to the power of an `Expression`.
    ///
    /// Mirrors the function signature of [`f64::exp`].
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(super) fn exp(self) -> Self {
        ExpExpr::new(self).into()
    }
}

#[derive(Debug)]
struct ExpExpr {
    data: f64,
    exponent: Expression,
}

impl ExpExpr {
    fn new(exponent: Expression) -> Self {
        let data = exponent.data().exp();

        Self { data, exponent }
    }
}

impl Expr for ExpExpr {
    ///
    /// Backward pass for natural exponentiation
    ///
    /// Follows from:
    ///
    /// ```text
    /// f(x)  = e^x
    /// df/dx = e^x
    /// ```
    ///
    /// By the chain rule, the exponent's gradient is
    /// `grad * e^x`. That is `grad` times this
    /// expression's own data.
    ///
    fn backward(&self, grad: f64) {
        self.exponent.add_grad(self.data * grad);
    }

    fn data(&self) -> f64 {
        self.data
    }

    fn left(&self) -> Option<&Expression> {
        Some(&self.exponent)
    }

    fn right(&self) -> Option<&Expression> {
        None
    }

    fn symbol(&self) -> &'static str {
        "e^"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp() {
        let x = Expression::from(2.0);
        let e = x.exp();

        assert_eq!(e.data(), 2.0_f64.exp(), "exp result data");
        assert_eq!(
            e.left().map(|l| l.data()),
            Some(2.0),
            "exp has left operand"
        );
        assert!(e.right().is_none(), "exp has no right operand");
        assert_eq!(e.symbol(), "e^", "exp has operator");
    }

    #[test]
    fn backward() {
        let e = Expression::from(2.0).exp();
        e.add_grad(2.0);
        e.backward();

        assert_eq!(
            e.left().map(Expression::grad),
            Some(2.0_f64.exp() * 2.0),
            "exp passes e^x gradient to exponent"
        );
    }
}
