use super::direction::Direction;
use super::point::Point;
use super::{Slot, World};
use crate::tuning::RESPAWN_MOVES;
use crate::turn::Turn;

///
/// A living snake.
///
/// Only its head is kept here. The cells of its tail
/// are on the board, each one a [`Tail`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Snake {
    pub(crate) head: Point,
    pub(crate) heading: Direction,

    /// The last cell of its tail, or its head if it has none.
    pub(crate) tail_end: Point,

    /// Moves since the snake last ate.
    pub(crate) hunger: u32,
}

/// One cell of a snake's tail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Tail {
    /// The slot of the snake it belongs to.
    pub(crate) snake: usize,

    /// The way to the snake's next cell, going toward its head.
    pub(crate) toward_head: Direction,
}

/// A snake's next move: where its head goes, and the way it then faces.
#[derive(Clone, Copy, Debug)]
pub(super) struct Move {
    pub(super) head: Point,
    pub(super) heading: Direction,
}

impl Snake {
    /// The move this snake makes by taking `turn`.
    pub(super) const fn next(&self, turn: Turn) -> Move {
        let heading = self.heading.turned(turn);

        Move {
            head: self.head.step(heading, 1),
            heading,
        }
    }
}

impl World {
    /// The piece of tail on `point`, if any.
    pub(crate) fn tail(&self, point: Point) -> Option<Tail> {
        let index = self.size.index(point)?;

        *self.tails.get(index)?
    }

    /// Puts `tail` on `point`, or clears the cell with `None`.
    pub(crate) fn set_tail(&mut self, point: Point, tail: Option<Tail>) {
        let cell = self
            .size
            .index(point)
            .and_then(|index| self.tails.get_mut(index));
        if let Some(cell) = cell {
            *cell = tail;
        }
    }

    ///
    /// Makes `step` with the snake in slot `index`.
    ///
    /// Its tail follows, unless there's food under its new
    /// head. Eating that grows it by one cell.
    ///
    pub(super) fn advance(&mut self, index: usize, step: Move) {
        let Some(mut snake) = self.snake(index).copied() else {
            return;
        };
        let neck = Tail {
            snake: index,
            toward_head: step.heading,
        };
        self.set_tail(snake.head, Some(neck));
        snake.head = step.head;
        snake.heading = step.heading;

        if self.food.contains(&step.head) {
            self.food.retain(|&food| food != step.head);
            snake.hunger = 0;
        } else {
            snake.tail_end = self.retract(snake.tail_end);
            snake.hunger += 1;
        }

        self.set_slot(index, Slot::Alive(snake));
    }

    ///
    /// Takes the snake in slot `index` off the board, and
    /// starts its wait to respawn.
    ///
    pub(super) fn kill(&mut self, index: usize) {
        let Some(&Snake { tail_end, .. }) = self.snake(index) else {
            return;
        };
        let mut end = tail_end;
        while self.tail(end).is_some() {
            end = self.retract(end);
        }

        self.set_slot(index, Slot::Respawning(RESPAWN_MOVES));
    }

    ///
    /// Clears the last cell of the tail ending at
    /// `tail_end`, and returns where the tail ends now.
    ///
    fn retract(&mut self, tail_end: Point) -> Point {
        let Some(tail) = self.tail(tail_end) else {
            return tail_end;
        };
        self.set_tail(tail_end, None);

        tail_end.step(tail.toward_head, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{points, snake, snake_cells, world};

    fn step(x: i32, y: i32, heading: Direction) -> Move {
        Move {
            head: Point { x, y },
            heading,
        }
    }

    #[test]
    fn next() {
        let world = world(10, 10, vec![snake(&[(5, 5), (4, 5)], Direction::East)]);
        let step = world.snake(0).unwrap().next(Turn::Left);

        assert_eq!(
            (step.head, step.heading),
            (Point { x: 5, y: 4 }, Direction::North),
            "next turns, then moves the head one cell that way"
        );
    }

    #[test]
    fn set_tail() {
        let mut world = world(10, 10, vec![]);
        let tail = Tail {
            snake: 0,
            toward_head: Direction::North,
        };
        world.set_tail(Point { x: 3, y: 4 }, Some(tail));
        world.set_tail(Point { x: 10, y: 4 }, Some(tail));

        assert_eq!(
            [(3, 4), (4, 4), (10, 4)].map(|(x, y)| world.tail(Point { x, y })),
            [Some(tail), None, None],
            "set_tail puts a tail on one cell, and ignores cells off the board"
        );
    }

    #[test]
    fn advance() {
        let mut world = world(
            10,
            10,
            vec![snake(&[(5, 5), (4, 5), (3, 5)], Direction::East)],
        );
        world.advance(0, step(5, 4, Direction::North));

        assert_eq!(
            snake_cells(&world, 0),
            points(&[(5, 4), (5, 5), (4, 5)]),
            "advance moves the head and the tail follows"
        );
        assert_eq!(
            (
                world.tail(Point { x: 3, y: 5 }),
                world.snake(0).unwrap().hunger
            ),
            (None, 1),
            "advance without eating leaves the last cell, and adds hunger"
        );
    }

    #[test]
    fn advance_eating() {
        let mut world = world(
            10,
            10,
            vec![snake(&[(5, 5), (4, 5), (3, 5)], Direction::East)],
        );
        world.food = points(&[(6, 5)]);
        if let Some(Slot::Alive(hungry)) = world.slots.first_mut() {
            hungry.hunger = 10;
        }
        world.advance(0, step(6, 5, Direction::East));

        assert_eq!(
            snake_cells(&world, 0),
            points(&[(6, 5), (5, 5), (4, 5), (3, 5)]),
            "advance onto food grows by one cell"
        );
        assert_eq!(
            (world.food.len(), world.snake(0).unwrap().hunger),
            (0, 0),
            "advance onto food eats it, and resets hunger"
        );
    }

    #[test]
    fn advance_no_tail() {
        let mut world = world(10, 10, vec![snake(&[(5, 5)], Direction::East)]);
        world.advance(0, step(6, 5, Direction::East));

        assert_eq!(
            snake_cells(&world, 0),
            points(&[(6, 5)]),
            "advance moves a snake with no tail without leaving one behind"
        );
    }

    #[test]
    fn kill() {
        let mut world = world(
            10,
            10,
            vec![
                snake(&[(5, 5), (4, 5), (4, 6)], Direction::East),
                snake(&[(2, 2), (2, 3)], Direction::North),
            ],
        );
        world.kill(0);

        assert_eq!(
            world.slots.first(),
            Some(&Slot::Respawning(4)),
            "kill starts the snake's wait to respawn"
        );
        assert_eq!(
            [(5, 5), (4, 5), (4, 6), (2, 3)].map(|(x, y)| world.blocked(Point { x, y })),
            [false, false, false, true],
            "kill clears the snake's cells, and no other snake's"
        );
    }
}
