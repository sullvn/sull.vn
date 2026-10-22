use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Div, Neg, Sub};

mod add;
mod exp;
mod lit;
mod mul;
mod pow;
mod tanh;

#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
trait Expr: Debug {
    fn backward(&mut self);

    fn data(&self) -> f64;
    fn grad(&self) -> f64;
    fn left(&self) -> Option<&Expression>;
    fn right(&self) -> Option<&Expression>;
    fn symbol(&self) -> &'static str;

    fn add_grad(&mut self, grad: f64);
}

#[derive(Debug)]
struct Expression(Box<dyn Expr>);

impl Deref for Expression {
    type Target = dyn Expr;

    fn deref(&self) -> &Self::Target {
        let Self(expr) = self;
        expr.as_ref()
    }
}

impl DerefMut for Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(expr) = self;
        expr.as_mut()
    }
}

impl<T: Expr + 'static> From<T> for Expression {
    fn from(expr: T) -> Self {
        Self(Box::new(expr))
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
