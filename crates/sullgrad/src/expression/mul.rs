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
    fn data(&self) -> f64 {
        self.data
    }

    fn symbol(&self) -> &'static str {
        "*"
    }

    fn left(&self) -> Option<&Expression> {
        Some(&self.left)
    }

    fn right(&self) -> Option<&Expression> {
        Some(&self.right)
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
}
