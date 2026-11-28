use super::point::Point;
use crate::turn::Turn;

///
/// A fixed direction on the board, named like a map.
///
/// North is the top of the screen. Unlike a [`Turn`], it
/// doesn't depend on which way a snake faces.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub(crate) const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    /// The direction after making `turn`.
    pub(crate) const fn turned(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.left(),
            Turn::Straight => self,
            Turn::Right => self.right(),
        }
    }

    /// The turn from this direction to `other`, unless `other` is straight behind.
    pub(crate) fn turn_to(self, other: Self) -> Option<Turn> {
        Turn::ALL
            .into_iter()
            .find(|&turn| self.turned(turn) == other)
    }

    /// The direction a quarter turn anticlockwise.
    pub(crate) const fn left(self) -> Self {
        self.opposite().right()
    }

    /// The direction straight behind.
    pub(crate) const fn opposite(self) -> Self {
        self.right().right()
    }

    /// The direction a quarter turn clockwise.
    pub(crate) const fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// One step in this direction.
    pub(crate) const fn offset(self) -> Point {
        match self {
            Self::North => Point { x: 0, y: -1 },
            Self::East => Point { x: 1, y: 0 },
            Self::South => Point { x: 0, y: 1 },
            Self::West => Point { x: -1, y: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turned() {
        assert_eq!(
            Turn::ALL.map(|turn| Direction::North.turned(turn)),
            [Direction::West, Direction::North, Direction::East],
            "turned turns left, keeps straight, or turns right"
        );
    }

    #[test]
    fn turn_to() {
        assert_eq!(
            Direction::ALL.map(|other| Direction::North.turn_to(other)),
            [
                Some(Turn::Straight),
                Some(Turn::Right),
                None,
                Some(Turn::Left),
            ],
            "turn_to finds the turn to each direction, except straight behind"
        );
    }

    #[test]
    fn left() {
        assert_eq!(
            Direction::ALL.map(Direction::left),
            [
                Direction::West,
                Direction::North,
                Direction::East,
                Direction::South,
            ],
            "left turns a quarter turn anticlockwise"
        );
    }

    #[test]
    fn opposite() {
        assert_eq!(
            Direction::ALL.map(Direction::opposite),
            [
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::East,
            ],
            "opposite faces straight behind"
        );
    }
}
