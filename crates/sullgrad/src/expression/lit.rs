use super::{Expr, Expression};

impl From<f64> for Expression {
    fn from(data: f64) -> Self {
        Lit::new(data).into()
    }
}

#[derive(Debug)]
struct Lit {
    data: f64,
}

impl Lit {
    const fn new(data: f64) -> Self {
        Self { data }
    }
}

impl Expr for Lit {
    ///
    /// Propagate gradient backward for literals
    ///
    /// There is nowhere to propagate to. Literals
    /// have no operands.
    ///
    fn propagate(&self, _grad: f64) {}

    fn data(&self) -> f64 {
        self.data
    }

    fn symbol(&self) -> &'static str {
        ""
    }

    fn left(&self) -> Option<&Expression> {
        None
    }

    fn right(&self) -> Option<&Expression> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit() {
        let l = Expression::from(1.0);

        assert_eq!(l.data(), 1.0, "literal data");
        assert!(l.left().is_none(), "literal has no left operand");
        assert!(l.right().is_none(), "literal has no right operand");
        assert_eq!(l.symbol(), "", "literal has no operator");
    }

    #[test]
    fn propagate() {
        let l = Expression::from(1.0);
        l.add_grad(2.0);
        l.propagate();

        assert_eq!(
            l.grad(),
            2.0,
            "literal propagates nothing, leaving its gradient unchanged"
        );
    }
}
