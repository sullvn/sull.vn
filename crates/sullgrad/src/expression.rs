use std::cell::Cell;
use std::fmt::Debug;
use std::ops::{Deref, Div, Neg, Sub};
use std::rc::Rc;

mod add;
mod exp;
mod lit;
mod mul;
mod pow;
mod tanh;

#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
trait Expr: Debug {
    fn backward(&self, grad: f64);

    fn data(&self) -> f64;
    fn left(&self) -> Option<&Expression>;
    fn right(&self) -> Option<&Expression>;
    fn symbol(&self) -> &'static str;
}

#[derive(Debug)]
struct Node<T: ?Sized> {
    grad: Cell<f64>,
    expr: T,
}

#[derive(Clone, Debug)]
struct Expression(Rc<Node<dyn Expr>>);

impl Expression {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    fn backward(&self) {
        let Self(node) = self;
        node.expr.backward(node.grad.get());
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    fn grad(&self) -> f64 {
        let Self(node) = self;
        node.grad.get()
    }

    fn add_grad(&self, grad: f64) {
        let Self(node) = self;
        node.grad.set(node.grad.get() + grad);
    }
}

impl Deref for Expression {
    type Target = dyn Expr;

    fn deref(&self) -> &Self::Target {
        let Self(node) = self;
        &node.expr
    }
}

impl<T: Expr + 'static> From<T> for Expression {
    fn from(expr: T) -> Self {
        let grad = Cell::new(0.0);

        Self(Rc::new(Node { grad, expr }))
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self {
        self * (-1.0).into()
    }
}

impl Sub<Self> for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Div<Self> for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.pow((-1.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        let x = Expression::from(2.0);
        let n = -x;

        assert_eq!(n.data(), -2.0, "neg result data");
        assert_eq!(
            n.left().map(|l| l.data()),
            Some(2.0),
            "neg has left operand"
        );
        assert_eq!(
            n.right().map(|r| r.data()),
            Some(-1.0),
            "neg has right operand"
        );
        assert_eq!(n.symbol(), "*", "neg has operator");
    }

    #[test]
    fn sub() {
        let x = Expression::from(5.0);
        let y = Expression::from(3.0);
        let s = x - y;

        assert_eq!(s.data(), 2.0, "sub result data");
        assert_eq!(
            s.left().map(|l| l.data()),
            Some(5.0),
            "sub has left operand"
        );
        assert_eq!(
            s.right().map(|r| r.data()),
            Some(-3.0),
            "sub has right operand"
        );
        assert_eq!(s.symbol(), "+", "sub has operator");
    }

    #[test]
    fn div() {
        let x = Expression::from(6.0);
        let y = Expression::from(2.0);
        let d = x / y;

        assert_eq!(d.data(), 3.0, "div result data");
        assert_eq!(
            d.left().map(|l| l.data()),
            Some(6.0),
            "div has left operand"
        );
        assert_eq!(
            d.right().map(|r| r.data()),
            Some(0.5),
            "div has right operand"
        );
        assert_eq!(d.symbol(), "*", "div has operator");
    }
}
