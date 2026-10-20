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
    fn from_constant() {
        let c = Expression::from(1.0);

        assert_eq!(c.data(), 1.0, "constant data");
        assert!(c.left().is_none(), "constant has no left operand");
        assert!(c.right().is_none(), "constant has no right operand");
        assert_eq!(c.symbol(), "", "constant has no operator");
    }
}
