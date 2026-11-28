use rand::Rng;

use super::{Direction, Point, Size, Snake, SnakeState, SnakeTail};

///
/// The board, its snakes, and their food.
///
/// Every snake moves at once, and food reappears as
/// it's eaten, one piece per snake.
///
#[derive(Clone, Debug)]
pub(crate) struct World {
    /// The board's width and height in cells.
    pub(crate) size: Size,

    /// Each snake's head and heading, or its wait to respawn.
    pub(crate) snakes: Vec<SnakeState>,

    /// The piece of tail on each cell, if any, row by row.
    pub(super) tails: Box<[Option<SnakeTail>]>,

    /// Where the food is.
    pub(crate) food: Vec<Point>,
}

impl World {
    /// A board full of new snakes and their food.
    pub(crate) fn new(size: Size, rng: &mut impl Rng) -> Self {
        let mut world = Self::empty(size);
        world.snakes = vec![SnakeState::Respawning(0); size.snake_count()];
        for index in 0..world.snakes.len() {
            world.respawn(index, rng);
        }
        world.add_food(rng);

        world
    }

    /// A board with no snakes or food.
    pub(crate) fn empty(size: Size) -> Self {
        Self {
            size,
            snakes: Vec::new(),
            tails: vec![None; size.area()].into_boxed_slice(),
            food: Vec::new(),
        }
    }

    /// Snake `index`, if it's alive.
    pub(crate) fn snake(&self, index: usize) -> Option<&Snake> {
        self.snakes.get(index).and_then(SnakeState::alive)
    }

    /// Every living snake.
    pub(crate) fn living(&self) -> impl Iterator<Item = &Snake> {
        self.snakes.iter().filter_map(SnakeState::alive)
    }

    /// Whether `point` is a wall or part of a living snake.
    pub(crate) fn is_blocked(&self, point: Point) -> bool {
        !self.size.contains(point)
            || self.tail(point).is_some()
            || self.living().any(|snake| snake.head == point)
    }

    /// Steps from the cell `from` in `direction` to the first wall or snake.
    pub(crate) fn steps_to_blocked(&self, from: Point, direction: Direction) -> i32 {
        let farthest = self.size.longest_side();

        (1..farthest)
            .find(|&steps| self.is_blocked(from.step(direction, steps)))
            .unwrap_or(farthest)
    }

    /// Whether `point` is on the board with no snake or food on it.
    pub(super) fn is_free(&self, point: Point) -> bool {
        !self.is_blocked(point) && !self.food.contains(&point)
    }

    /// Replaces the state of snake `index`.
    pub(super) fn set_snake(&mut self, index: usize, state: SnakeState) {
        if let Some(old) = self.snakes.get_mut(index) {
            *old = state;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{seeded, snake, world};

    #[test]
    fn new() {
        let world = World::new(Size::new(20, 28).unwrap(), &mut seeded(1));

        assert_eq!(
            (world.snakes.len(), world.living().count(), world.food.len()),
            (3, 3, 3),
            "new spawns every snake, with one piece of food each"
        );
    }

    #[test]
    fn steps_to_blocked() {
        let world = world(
            10,
            10,
            vec![
                snake(&[(5, 5), (5, 6)], Direction::North),
                snake(&[(8, 5)], Direction::North),
            ],
        );
        let from = Point { x: 5, y: 5 };

        assert_eq!(
            Direction::ALL.map(|direction| world.steps_to_blocked(from, direction)),
            [6, 3, 1, 6],
            "steps_to_blocked counts the steps to the first wall or snake"
        );
    }
}
