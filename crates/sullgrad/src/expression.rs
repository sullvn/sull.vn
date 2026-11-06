use std::cell::Cell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

mod add;
mod backward;
mod div;
mod exp;
mod lit;
mod mul;
mod neg;
mod pow;
mod sub;
mod sum;
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
