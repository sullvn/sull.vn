use std::cell::Cell;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Deref, Div, Neg, Sub};
use std::rc::Rc;

mod add;
mod backward;
mod exp;
mod lit;
mod mul;
mod pow;
mod tanh;
mod walk;

#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) trait Expr: Debug {
    fn propagate(&self, grad: f64);

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
pub(crate) struct Expression(Rc<Node<dyn Expr>>);

impl Expression {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    fn propagate(&self) {
        let Self(node) = self;
        node.expr.propagate(node.grad.get());
    }

    pub(crate) fn data(&self) -> f64 {
        let Self(node) = self;
        node.expr.data()
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn grad(&self) -> f64 {
        let Self(node) = self;
        node.grad.get()
    }

    fn add_grad(&self, grad: f64) {
        let Self(node) = self;
        node.grad.set(node.grad.get() + grad);
    }

    ///
    /// Seeds a backward pass.
    ///
    /// An expression's gradient with respect to itself
    /// is `1`.
    ///
    fn seed_grad(&self) {
        let Self(node) = self;
        node.grad.set(1.0);
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

impl Sum for Expression {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(0.0.into(), Add::add)
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
            n.left().map(Expression::data),
            Some(2.0),
            "neg has left operand"
        );
        assert_eq!(
            n.right().map(Expression::data),
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
            s.left().map(Expression::data),
            Some(5.0),
            "sub has left operand"
        );
        assert_eq!(
            s.right().map(Expression::data),
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
            d.left().map(Expression::data),
            Some(6.0),
            "div has left operand"
        );
        assert_eq!(
            d.right().map(Expression::data),
            Some(0.5),
            "div has right operand"
        );
        assert_eq!(d.symbol(), "*", "div has operator");
    }

    #[test]
    fn sum() {
        let s: Expression = [1.0, 2.0, 3.0].map(Expression::from).into_iter().sum();

        assert_eq!(s.data(), 6.0, "sum result data");
        assert_eq!(s.symbol(), "+", "sum has operator");
    }

    #[test]
    fn empty_sum() {
        let s: Expression = std::iter::empty().sum();

        assert_eq!(s.data(), 0.0, "empty sum is zero");
    }
}
