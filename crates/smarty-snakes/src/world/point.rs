use super::direction::Direction;

///
/// A cell's position on the board.
///
/// `y` grows downward, like rows on a screen. Points can
/// lie off the board, which is how walls are found.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    /// The point `steps` cells away in `direction`.
    pub(crate) const fn step(self, direction: Direction, steps: i32) -> Self {
        let Self { x, y } = direction.offset();

        Self {
            x: self.x + x * steps,
            y: self.y + y * steps,
        }
    }

    ///
    /// Cells between two points, moving only along rows
    /// and columns.
    ///
    /// Also known as the Manhattan distance.
    ///
    pub(crate) const fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    ///
    /// Where `other` is, as seen facing `heading`.
    ///
    /// Returns cells ahead, then cells to the right.
    /// Behind and to the left are negative.
    ///
    pub(crate) const fn relative(self, heading: Direction, other: Self) -> (i32, i32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let ahead = heading.offset();
        let right = heading.right().offset();

        (dx * ahead.x + dy * ahead.y, dx * right.x + dy * right.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step() {
        let point = Point { x: 5, y: 5 };

        assert_eq!(
            Direction::ALL.map(|direction| point.step(direction, 2)),
            [
                Point { x: 5, y: 3 },
                Point { x: 7, y: 5 },
                Point { x: 5, y: 7 },
                Point { x: 3, y: 5 },
            ],
            "step moves along the direction, with up toward row zero"
        );
    }

    #[test]
    fn distance() {
        assert_eq!(
            Point { x: 1, y: 2 }.distance(Point { x: 4, y: 0 }),
            5,
            "distance adds the row and column differences"
        );
    }

    #[test]
    fn relative() {
        let point = Point { x: 5, y: 5 };
        let other = Point { x: 7, y: 4 };

        assert_eq!(
            Direction::ALL.map(|heading| point.relative(heading, other)),
            [(1, 2), (2, -1), (-1, -2), (-2, 1)],
            "relative turns offsets to ahead and right of the heading"
        );
    }
}
