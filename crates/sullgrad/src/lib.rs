//! A humble micrograd implementation
use std::ops::{Add, Deref};

#[derive(Debug, PartialEq)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
struct ValueFields {
    data: f64,
    left: Option<Value>,
    right: Option<Value>,
    op: &'static str,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
struct Value(Box<ValueFields>);

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
            op: "",
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
            op: "+",
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_constant() {
        let value = Value::from(1.0);

        assert_eq!(value.data, 1.0, "constant value");
        assert_eq!(value.left, None, "constant has no left operand");
        assert_eq!(value.right, None, "constant has no right operand");
        assert_eq!(value.op, "", "constant has no operator");
    }

    #[test]
    fn add() {
        let x = Value::from(1.0);
        let y = Value::from(2.0);
        let result = x + y;

        assert_eq!(result.data, 3.0, "correct addition computation");
    }
}
