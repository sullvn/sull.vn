//! A humble micrograd implementation
use std::borrow::Cow;
use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
struct ValueFields {
    data: f64,
    left: Option<Value>,
    right: Option<Value>,
    op: Cow<'static, str>,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
struct Value(Box<ValueFields>);

impl Value {
    ///
    /// Raises a `Value` to a floating point power.
    ///
    /// Mirrors the function signature of [`f64::powf`].
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    fn powf(self, n: Self) -> Self {
        let data = self.data.powf(n.data);

        Self(Box::new(ValueFields {
            data,
            left: Some(self),
            right: Some(n),
            op: "**".into(),
        }))
    }
}

impl Deref for Value {
    type Target = ValueFields;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Value {
    fn from(data: f64) -> Self {
        Self(Box::new(ValueFields {
            data,
            left: None,
            right: None,
            op: "".into(),
        }))
    }
}

impl Add<Self> for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let data = self.data + rhs.data;

        Self(Box::new(ValueFields {
            data,
            left: Some(self),
            right: Some(rhs),
            op: "+".into(),
        }))
    }
}

impl Mul<Self> for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let data = self.data * rhs.data;

        Self(Box::new(ValueFields {
            data,
            left: Some(self),
            right: Some(rhs),
            op: "*".into(),
        }))
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        self * (-1.0).into()
    }
}

impl Sub<Self> for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Div<Self> for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.powf((-1.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_constant() {
        let c = Value::from(1.0);

        assert_eq!(c.data, 1.0, "constant data");
        assert_eq!(c.left, None, "constant has no left operand");
        assert_eq!(c.right, None, "constant has no right operand");
        assert_eq!(c.op, "", "constant has no operator");
    }

    #[test]
    fn add() {
        let x = Value::from(1.0);
        let y = Value::from(2.0);
        let a = x + y;

        assert_eq!(a.data, 3.0, "add result data");
        assert_eq!(
            a.left.as_ref().map(|l| l.data),
            Some(1.0),
            "add has left operand"
        );
        assert_eq!(
            a.right.as_ref().map(|l| l.data),
            Some(2.0),
            "add has right operand"
        );
        assert_eq!(a.op, "+", "add has operator");
    }

    #[test]
    fn mul() {
        let x = Value::from(2.0);
        let y = Value::from(3.0);
        let m = x * y;

        assert_eq!(m.data, 6.0, "mul result data");
        assert_eq!(
            m.left.as_ref().map(|l| l.data),
            Some(2.0),
            "mul has left operand"
        );
        assert_eq!(
            m.right.as_ref().map(|l| l.data),
            Some(3.0),
            "mul has right operand"
        );
        assert_eq!(m.op, "*", "mul has operator");
    }

    #[test]
    fn powf() {
        let x = Value::from(2.0);
        let y = Value::from(3.0);
        let m = x.powf(y);

        assert_eq!(m.data, 8.0, "powf result data");
        assert_eq!(
            m.left.as_ref().map(|l| l.data),
            Some(2.0),
            "powf has left operand"
        );
        assert_eq!(
            m.right.as_ref().map(|l| l.data),
            Some(3.0),
            "powf has right operand"
        );
        assert_eq!(m.op, "**", "powf has operator");
    }

    #[test]
    fn neg() {
        let x = Value::from(2.0);
        let n = -x;

        assert_eq!(n.data, -2.0, "neg result data");
        assert_eq!(
            n.left.as_ref().map(|l| l.data),
            Some(2.0),
            "neg has left operand"
        );
        assert_eq!(
            n.right.as_ref().map(|l| l.data),
            Some(-1.0),
            "neg has right operand"
        );
        assert_eq!(n.op, "*", "neg has operator");
    }

    #[test]
    fn sub() {
        let x = Value::from(5.0);
        let y = Value::from(3.0);
        let s = x - y;

        assert_eq!(s.data, 2.0, "sub result data");
        assert_eq!(
            s.left.as_ref().map(|l| l.data),
            Some(5.0),
            "sub has left operand"
        );
        assert_eq!(
            s.right.as_ref().map(|l| l.data),
            Some(-3.0),
            "sub has right operand"
        );
        assert_eq!(s.op, "+", "sub has operator");
    }

    #[test]
    fn div() {
        let x = Value::from(6.0);
        let y = Value::from(2.0);
        let d = x / y;

        assert_eq!(d.data, 3.0, "div result data");
        assert_eq!(
            d.left.as_ref().map(|l| l.data),
            Some(6.0),
            "div has left operand"
        );
        assert_eq!(
            d.right.as_ref().map(|l| l.data),
            Some(0.5),
            "div has right operand"
        );
        assert_eq!(d.op, "*", "div has operator");
    }
}
