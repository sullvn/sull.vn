use super::point::Point;
use crate::turn::Turn;

/// A direction on the board, as seen on screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub(crate) const ALL: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    /// The direction after making `turn`.
    pub(crate) const fn turned(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.right().right().right(),
            Turn::Straight => self,
            Turn::Right => self.right(),
        }
    }

    /// The direction a quarter turn clockwise.
    pub(crate) const fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// One step in this direction.
    pub(crate) const fn offset(self) -> Point {
        match self {
            Self::Up => Point { x: 0, y: -1 },
            Self::Right => Point { x: 1, y: 0 },
            Self::Down => Point { x: 0, y: 1 },
            Self::Left => Point { x: -1, y: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turned() {
        assert_eq!(
            Turn::ALL.map(|turn| Direction::Up.turned(turn)),
            [Direction::Left, Direction::Up, Direction::Right],
            "turned turns left, keeps straight, or turns right"
        );
    }
}
