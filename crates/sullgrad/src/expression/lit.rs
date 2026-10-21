use super::{Expr, Expression};

impl From<f64> for Expression {
    fn from(data: f64) -> Self {
        Lit::new(data).into()
    }
}

#[derive(Debug)]
struct Lit {
    data: f64,
    grad: f64,
}

impl Lit {
    const fn new(data: f64) -> Self {
        let grad = 0.0;

        Self { data, grad }
    }
}

impl Expr for Lit {
    ///
    /// Backward pass for literals
    ///
    /// There is none. Literals have nothing they can
    /// propagate gradients backwards to.
    ///
    fn backward(&mut self) {}

    fn data(&self) -> f64 {
        self.data
    }

    fn grad(&self) -> f64 {
        self.grad
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

    fn add_grad(&mut self, grad: f64) {
        self.grad += grad;
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
    fn backward() {
        let mut l = Expression::from(1.0);
        l.add_grad(2.0);
        l.backward();

        assert_eq!(l.grad(), 2.0, "literal backward leaves gradient unchanged");
    }
}
