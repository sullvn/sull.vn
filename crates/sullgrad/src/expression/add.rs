use std::ops::Add;

use super::{Expr, Expression};

impl Add<Self> for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        AddExpr::new(self, rhs).into()
    }
}

#[derive(Debug)]
struct AddExpr {
    data: f64,
    left: Expression,
    right: Expression,
}

impl AddExpr {
    fn new(left: Expression, right: Expression) -> Self {
        let data = left.data() + right.data();

        Self { data, left, right }
    }
}

impl Expr for AddExpr {
    fn data(&self) -> f64 {
        self.data
    }

    fn symbol(&self) -> &'static str {
        "+"
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
    fn add() {
        let x = Expression::from(1.0);
        let y = Expression::from(2.0);
        let a = x + y;

        assert_eq!(a.data(), 3.0, "add result data");
        assert_eq!(
            a.left().map(|l| l.data()),
            Some(1.0),
            "add has left operand"
        );
        assert_eq!(
            a.right().map(|r| r.data()),
            Some(2.0),
            "add has right operand"
        );
        assert_eq!(a.symbol(), "+", "add has operator");
    }
}
