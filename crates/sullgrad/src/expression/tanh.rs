use super::{Expr, Expression};

impl Expression {
    ///
    /// Hyperbolic tangent of an `Expression`.
    ///
    /// Mirrors the function signature of [`f64::tanh`].
    ///
    /// A smooth S-curve from `-1` to `1`. Usually
    /// better than sigmoid, but usually worse than
    /// `ReLU` in deep networks.
    ///
    /// # Compared to sigmoid
    ///
    /// Sigmoid, `1 / (1 + e^-x)`, has the same S-shape,
    /// but networks using tanh usually learn faster:
    ///
    /// - Its output is centered on `0`. Sigmoid's is
    ///   always positive, so it nudges the next layer's
    ///   weights all in the same direction.
    /// - Its slope is up to 4x steeper, so it passes
    ///   back more of the gradient.
    ///
    /// # Compared to `ReLU`
    ///
    /// `ReLU`, `max(0, x)`, is usually better in deep
    /// networks. Tanh flattens out for large inputs,
    /// shrinking the gradient at every layer it passes
    /// through. `ReLU` doesn't flatten for positive
    /// inputs, and it's cheaper.
    ///
    /// Tanh is still the better fit when:
    ///
    /// - Output must stay between `-1` and `1`.
    /// - Output is fed back in as input step after step,
    ///   where `ReLU`'s unlimited output can grow out of
    ///   control.
    /// - The network is small, so tanh's flattening
    ///   costs less than `ReLU` units getting stuck at
    ///   `0` and no longer learning.
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(super) fn tanh(self) -> Self {
        TanhExpr::new(self).into()
    }
}

#[derive(Debug)]
struct TanhExpr {
    data: f64,
    grad: f64,
    operand: Expression,
}

impl TanhExpr {
    fn new(operand: Expression) -> Self {
        let data = operand.data().tanh();
        let grad = 0.0;

        Self {
            data,
            grad,
            operand,
        }
    }
}

impl Expr for TanhExpr {
    ///
    /// Backward pass for hyperbolic tangent
    ///
    /// Follows from:
    ///
    /// ```text
    /// f(x)  = tanh(x)
    /// df/dx = 1 - tanh(x)^2
    /// ```
    ///
    /// By the chain rule, the operand's gradient is
    /// `self.grad * (1 - tanh(x)^2)`. Since `tanh(x)`
    /// is this expression's own data, nothing needs
    /// recomputing.
    ///
    fn backward(&mut self) {
        let t = self.data;

        self.operand.add_grad(t.mul_add(-t, 1.0) * self.grad);
    }

    fn data(&self) -> f64 {
        self.data
    }

    fn grad(&self) -> f64 {
        self.grad
    }

    fn left(&self) -> Option<&Expression> {
        Some(&self.operand)
    }

    fn right(&self) -> Option<&Expression> {
        None
    }

    fn symbol(&self) -> &'static str {
        "tanh"
    }

    fn add_grad(&mut self, grad: f64) {
        self.grad += grad;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tanh() {
        let x = Expression::from(2.0);
        let t = x.tanh();

        assert_eq!(t.data(), 2.0_f64.tanh(), "tanh result data");
        assert_eq!(
            t.left().map(|l| l.data()),
            Some(2.0),
            "tanh has left operand"
        );
        assert!(t.right().is_none(), "tanh has no right operand");
        assert_eq!(t.symbol(), "tanh", "tanh has operator");
    }

    #[test]
    fn backward() {
        let mut t = Expression::from(2.0).tanh();
        t.add_grad(2.0);
        t.backward();

        let tanh = 2.0_f64.tanh();
        assert_eq!(
            t.left().map(|l| l.grad()),
            Some(tanh.mul_add(-tanh, 1.0) * 2.0),
            "tanh passes 1 - tanh(x)^2 gradient to operand"
        );
    }
}
