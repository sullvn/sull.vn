use super::Expression;

impl Expression {
    ///
    /// Backpropagation.
    ///
    /// Computes the gradient of `self` with respect to
    /// every sub-expression.
    ///
    /// Named after `PyTorch`'s `Tensor.backward`.
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn backward(&self) {
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
        //
        // To help visualize the following results:
        //
        //     c = 1c
        //
        //       = a + b
        //       = a + 3a
        //       = 4a
        //
        //       = 4(2x)
        //       = 8x
        //
        let x = Expression::from(5.0);
        let a = x.clone() * Expression::from(2.0);
        let b = a.clone() * Expression::from(3.0);
        let c = a.clone() + b;
        c.backward();

        assert_eq!(c.grad(), 1.0, "backward seeds a gradient of one on `self`");
        assert_eq!(a.grad(), 4.0, "backward sums gradients from shared uses");
        assert_eq!(x.grad(), 8.0, "backward chains gradients through layers");
    }

    #[test]
    fn accumulates() {
        let x = Expression::from(5.0);
        let c = x.clone() * Expression::from(2.0);

        c.backward();
        c.backward();

        assert_eq!(x.grad(), 4.0, "backward accumulates across passes");
    }
}
