use std::ops::Mul;

use super::{Expr, Expression};

impl Mul<Self> for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        MulExpr::new(self, rhs).into()
    }
}

#[derive(Debug)]
struct MulExpr {
    data: f64,
    left: Expression,
    right: Expression,
}

impl MulExpr {
    fn new(left: Expression, right: Expression) -> Self {
        let data = left.data() * right.data();

        Self { data, left, right }
    }
}

impl Expr for MulExpr {
    ///
    /// Propagate gradient backward for multiplication
    ///
    /// Follows from:
    ///
    /// ```text
    /// f(l, r) = l * r
    /// ∂f/∂l   = r
    /// ∂f/∂r   = l
    /// ```
    ///
    /// By the chain rule, each operand's gradient is
    /// `grad` times the other operand's data.
    ///
    fn propagate(&self, grad: f64) {
        let (l, r) = (self.left.data(), self.right.data());

        self.left.add_grad(r * grad);
        self.right.add_grad(l * grad);
    }

    fn data(&self) -> f64 {
        self.data
    }

    fn left(&self) -> Option<&Expression> {
        Some(&self.left)
    }

    fn right(&self) -> Option<&Expression> {
        Some(&self.right)
    }

    fn symbol(&self) -> &'static str {
        "*"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let x = Expression::from(2.0);
        let y = Expression::from(3.0);
        let m = x * y;

        assert_eq!(m.data(), 6.0, "mul result data");
        assert_eq!(
            m.left().map(|l| l.data()),
            Some(2.0),
            "mul has left operand"
        );
        assert_eq!(
            m.right().map(|r| r.data()),
            Some(3.0),
            "mul has right operand"
        );
        assert_eq!(m.symbol(), "*", "mul has operator");
    }

    #[test]
    fn propagate() {
        let m = Expression::from(2.0) * Expression::from(3.0);
        m.add_grad(2.0);
        m.propagate();

        assert_eq!(
            m.left().map(Expression::grad),
            Some(6.0),
            "mul scales left gradient by right operand"
        );
        assert_eq!(
            m.right().map(Expression::grad),
            Some(4.0),
            "mul scales right gradient by left operand"
        );
    }
}
