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
    base: Expression,
    exponent: Expression,
}

impl PowExpr {
    fn new(base: Expression, exponent: Expression) -> Self {
        let data = base.data().powf(exponent.data());

        Self {
            data,
            base,
            exponent,
        }
    }
}

impl Expr for PowExpr {
    fn data(&self) -> f64 {
        self.data
    }

    fn symbol(&self) -> &'static str {
        "**"
    }

    fn left(&self) -> Option<&Expression> {
        Some(&self.base)
    }

    fn right(&self) -> Option<&Expression> {
        Some(&self.exponent)
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
}
