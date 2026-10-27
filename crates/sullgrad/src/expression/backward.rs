use super::Expression;

impl Expression {
    ///
    /// Backpropagation.
    ///
    /// Computes the gradient of `self` with respect to
    /// every sub-expression.
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(super) fn backward(&self) {
        self.seed_grad();
        for expression in self.walk() {
            expression.propagate();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backward() {
        let x = Expression::from(5.0);
        let a = x.clone() * Expression::from(2.0);
        let b = a.clone() * Expression::from(3.0);
        let c = a.clone() + b;
        c.backward();

        assert_eq!(a.grad(), 4.0, "backward sums gradients from shared uses");
        assert_eq!(x.grad(), 8.0, "backward chains gradients through layers");
    }
}
