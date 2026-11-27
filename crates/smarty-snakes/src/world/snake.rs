use std::collections::VecDeque;

use super::direction::Direction;
use super::point::Point;
use crate::tuning::SPAWN_LENGTH;

/// A living snake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Snake {
    pub(crate) head: Point,
    pub(crate) heading: Direction,

    /// Cells behind the head, from neck to tail.
    pub(crate) body: VecDeque<Point>,

    /// Moves since the snake last ate.
    pub(crate) hunger: u32,
}

impl Snake {
    /// A new snake with its head at `head`, facing `heading`.
    pub(crate) fn spawn(head: Point, heading: Direction) -> Self {
        let behind = heading.right().right();

        Self {
            head,
            heading,
            body: (1..SPAWN_LENGTH)
                .map(|steps| head.step(behind, steps))
                .collect(),
            hunger: 0,
        }
    }

    /// Every cell the snake covers, from head to tail.
    pub(crate) fn cells(&self) -> impl Iterator<Item = Point> {
        std::iter::once(self.head).chain(self.body.iter().copied())
    }

    ///
    /// Moves the head into `head`, now facing `heading`.
    ///
    /// The tail follows unless the snake is `eating`, which
    /// grows it by one cell.
    ///
    pub(crate) fn advance(&mut self, head: Point, heading: Direction, eating: bool) {
        self.body.push_front(self.head);
        self.head = head;
        self.heading = heading;

        if eating {
            self.hunger = 0;
        } else {
            self.body.pop_back();
            self.hunger += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::points;

    #[test]
    fn spawn() {
        let snake = Snake::spawn(Point { x: 5, y: 5 }, Direction::Right);

        assert_eq!(
            snake.cells().collect::<Vec<_>>(),
            points(&[(5, 5), (4, 5), (3, 5)]),
            "spawn makes a snake 3 cells long, trailing behind its heading"
        );
    }

    #[test]
    fn advance() {
        let mut snake = Snake::spawn(Point { x: 5, y: 5 }, Direction::Right);
        snake.advance(Point { x: 5, y: 4 }, Direction::Up, false);

        assert_eq!(
            snake.cells().collect::<Vec<_>>(),
            points(&[(5, 4), (5, 5), (4, 5)]),
            "advance moves the head and the tail follows"
        );
        assert_eq!(snake.hunger, 1, "advance without eating adds hunger");
    }

    #[test]
    fn advance_eating() {
        let mut snake = Snake::spawn(Point { x: 5, y: 5 }, Direction::Right);
        snake.hunger = 10;
        snake.advance(Point { x: 6, y: 5 }, Direction::Right, true);

        assert_eq!(
            snake.cells().collect::<Vec<_>>(),
            points(&[(6, 5), (5, 5), (4, 5), (3, 5)]),
            "advance while eating grows by one cell"
        );
        assert_eq!(snake.hunger, 0, "advance while eating resets hunger");
    }
}
