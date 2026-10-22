use std::collections::HashSet;
use std::rc::Rc;

use super::Expression;

enum Visit {
    Enter(Expression),
    Exit(Expression),
}

impl Expression {
    ///
    /// `self` and its sub-expressions, in topological
    /// order. Visits results before operands.
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(super) fn walk(&self) -> impl Iterator<Item = Self> {
        let mut seen = HashSet::new();
        let mut order = Vec::new();
        let mut stack = vec![Visit::Enter(self.clone())];

        while let Some(visit) = stack.pop() {
            match visit {
                Visit::Enter(expr) => {
                    if seen.insert(expr.id()) {
                        stack.push(Visit::Exit(expr.clone()));
                        stack.extend(expr.operands().map(Visit::Enter));
                    }
                }
                Visit::Exit(expr) => order.push(expr),
            }
        }

        order.into_iter().rev()
    }

    fn operands(&self) -> impl Iterator<Item = Self> {
        [self.left(), self.right()].into_iter().flatten().cloned()
    }

    fn id(&self) -> usize {
        let Self(node) = self;
        Rc::as_ptr(node).addr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk() {
        let x = Expression::from(5.0);
        let a = x * Expression::from(2.0);
        let b = a.clone() * Expression::from(3.0);
        let c = a + b;
        let order: Vec<f64> = c.walk().map(|e| e.data()).collect();

        assert_eq!(
            order,
            vec![40.0, 30.0, 10.0, 5.0, 2.0, 3.0],
            "walk visits results before operands"
        );
    }

    #[test]
    fn unique_visits() {
        let x = Expression::from(3.0);
        let y = x.clone() * x;

        assert_eq!(y.walk().count(), 2, "walk visits a shared expression once");
    }
}
