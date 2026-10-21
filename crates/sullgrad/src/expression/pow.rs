use super::{Expr, Expression};

impl Expression {
    ///
    /// Raises an `Expression` to a floating point power.
    ///
    /// Mirrors the function signature of [`f64::powf`].
    ///
    pub(super) fn pow(self, n: Self) -> Self {
        PowExpr::new(self, n).into()
    }
}

#[derive(Debug)]
struct PowExpr {
    data: f64,
    grad: f64,
    base: Expression,
    exponent: Expression,
}

impl PowExpr {
    fn new(base: Expression, exponent: Expression) -> Self {
        let data = base.data().powf(exponent.data());
        let grad = 0.0;

        Self {
            data,
            grad,
            base,
            exponent,
        }
    }
}

impl Expr for PowExpr {
    ///
    /// Backward pass for exponentiation
    ///
    /// Follows from:
    ///
    /// ```text
    /// f(b, e) = b^e
    /// ∂f/∂b   = e * b^(e - 1)
    /// ∂f/∂e   = ln(b) * b^e
    /// ```
    ///
    /// By the chain rule, the base's gradient is
    /// `self.grad * ∂f/∂b`, while the exponent's
    /// gradient is `self.grad * ∂f/∂e`.
    ///
    ///
    /// # Warning
    ///
    /// If `b <= 0`, the exponent's gradient is NaN or
    /// infinite. This is from `ln(b)` being undefined
    /// for non-positive values.
    ///
    /// Division by a negative number hits this, but
    /// it's fine. Nothing reads the gradient of its
    /// `-1` exponent.
    ///
    fn backward(&mut self) {
        let (b, e) = (self.base.data(), self.exponent.data());

        self.base.add_grad(e * b.powf(e - 1.0) * self.grad);
        self.exponent.add_grad(b.ln() * self.data * self.grad);
    }

    fn data(&self) -> f64 {
        self.data
    }

    fn grad(&self) -> f64 {
        self.grad
    }

    fn left(&self) -> Option<&Expression> {
        Some(&self.base)
    }

    fn right(&self) -> Option<&Expression> {
        Some(&self.exponent)
    }

    fn symbol(&self) -> &'static str {
        "**"
    }

    fn add_grad(&mut self, grad: f64) {
        self.grad += grad;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow() {
        let x = Expression::from(2.0);
        let y = Expression::from(3.0);
        let p = x.pow(y);

        assert_eq!(p.data(), 8.0, "pow result data");
        assert_eq!(
            p.left().map(|l| l.data()),
            Some(2.0),
            "pow has left operand"
        );
        assert_eq!(
            p.right().map(|r| r.data()),
            Some(3.0),
            "pow has right operand"
        );
        assert_eq!(p.symbol(), "**", "pow has operator");
    }

    #[test]
    fn backward() {
        let mut p = Expression::from(2.0).pow(Expression::from(3.0));
        p.add_grad(2.0);
        p.backward();

        assert_eq!(
            p.left().map(|l| l.grad()),
            Some(24.0),
            "pow passes e * b^(e - 1) gradient to base"
        );
        assert_eq!(
            p.right().map(|r| r.grad()),
            Some(2.0_f64.ln() * 8.0 * 2.0),
            "pow passes ln(b) * b^e gradient to exponent"
        );
    }
}
