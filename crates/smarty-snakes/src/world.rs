use std::collections::HashSet;

use crate::tuning::{RESPAWN_MOVES, STARVATION_MOVES};
use crate::turn::Turn;

mod board;
mod cells;
mod direction;
mod food;
mod point;
mod snake;
mod spawn;

pub(crate) use board::Board;
pub(crate) use direction::Direction;
pub(crate) use point::Point;
pub(crate) use snake::Snake;

/// The slot of the snake you steer. The rest are clones.
pub(crate) const YOU: usize = 0;

///
/// The board, its snakes, and their food.
///
/// Every snake moves at once, and food reappears as
/// it's eaten, one piece per snake.
///
#[derive(Clone, Debug)]
pub(crate) struct World {
    pub(crate) board: Board,
    pub(crate) slots: Vec<Slot>,
    pub(crate) food: Vec<Point>,
}

/// A place for one snake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Slot {
    Alive(Snake),

    /// Dead, with this many moves until it reappears.
    Respawning(u32),
}

impl World {
    /// A board full of new snakes and their food.
    pub(crate) fn new(board: Board, random: &mut impl FnMut() -> f64) -> Self {
        let mut world = Self {
            board,
            slots: vec![Slot::Respawning(0); board.snake_count()],
            food: Vec::new(),
        };
        for index in 0..world.slots.len() {
            world.respawn(index, random);
        }
        world.add_food(random);

        world
    }

    /// The living snake in slot `index`.
    pub(crate) fn snake(&self, index: usize) -> Option<&Snake> {
        self.slots.get(index).and_then(Slot::alive)
    }

    /// Every living snake.
    pub(crate) fn snakes(&self) -> impl Iterator<Item = &Snake> {
        self.slots.iter().filter_map(Slot::alive)
    }

    /// Whether `point` is a wall or part of a living snake.
    pub(crate) fn blocked(&self, point: Point) -> bool {
        !self.board.contains(point) || self.snakes().any(|snake| snake.cells().any(|c| c == point))
    }

    ///
    /// Moves every living snake at once.
    ///
    /// `turns` has a move for each living snake's slot.
    ///
    /// A snake dies if its new head is off the board, on a
    /// cell any snake covered before the move, or on
    /// another snake's new head. So moving into a tail
    /// that's about to leave is deadly, and so is swapping
    /// places head to head.
    ///
    /// Snakes also die of hunger. The dead leave at once
    /// and start counting down to respawn.
    ///
    pub(crate) fn step(&mut self, turns: &[Option<Turn>], random: &mut impl FnMut() -> f64) {
        let occupied = self.occupied();
        let moves: Vec<_> = self
            .slots
            .iter()
            .zip(turns)
            .map(|(slot, turn)| {
                let snake = slot.alive()?;
                let heading = snake.heading.turned((*turn)?);
                Some((snake.head.step(heading, 1), heading))
            })
            .collect();
        let heads: Vec<_> = moves.iter().map(|m| m.map(|(head, _)| head)).collect();

        for (index, (slot, step)) in self.slots.iter_mut().zip(&moves).enumerate() {
            let (Slot::Alive(snake), Some((head, heading))) = (&mut *slot, *step) else {
                continue;
            };
            let collides = !self.board.contains(head)
                || occupied.contains(&head)
                || heads
                    .iter()
                    .enumerate()
                    .any(|(other, other_head)| other != index && *other_head == Some(head));
            if collides {
                *slot = Slot::Respawning(RESPAWN_MOVES);
                continue;
            }

            let eating = self.food.contains(&head);
            self.food.retain(|&food| food != head);
            snake.advance(head, heading, eating);
            if snake.hunger >= STARVATION_MOVES {
                *slot = Slot::Respawning(RESPAWN_MOVES);
            }
        }

        self.add_food(random);
    }

    /// Cells covered by a living snake.
    fn occupied(&self) -> HashSet<Point> {
        self.snakes().flat_map(Snake::cells).collect()
    }

    /// Cells covered by a living snake or food.
    fn taken(&self) -> HashSet<Point> {
        let mut taken = self.occupied();
        taken.extend(&self.food);

        taken
    }
}

impl Slot {
    /// The snake, if it's alive.
    pub(crate) const fn alive(&self) -> Option<&Snake> {
        match self {
            Self::Alive(snake) => Some(snake),
            Self::Respawning(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{points, scripted, snake};

    fn world(snakes: Vec<Snake>) -> World {
        World {
            board: Board::new(10, 10).unwrap(),
            slots: snakes.into_iter().map(Slot::Alive).collect(),
            food: Vec::new(),
        }
    }

    fn step(world: &mut World, turns: &[Turn]) {
        let turns: Vec<_> = turns.iter().copied().map(Some).collect();
        world.step(&turns, &mut scripted(&[0.0]));
    }

    fn alive(world: &World) -> Vec<bool> {
        world
            .slots
            .iter()
            .map(|slot| slot.alive().is_some())
            .collect()
    }

    #[test]
    fn new() {
        let world = World::new(Board::new(20, 28).unwrap(), &mut scripted(&[0.3, 0.7]));

        assert_eq!(
            (alive(&world), world.food.len()),
            (vec![true; 3], 3),
            "new spawns every snake, with one piece of food each"
        );
    }

    #[test]
    fn moves() {
        let mut world = world(vec![snake(&[(5, 5), (4, 5), (3, 5)], Direction::Right)]);
        step(&mut world, &[Turn::Left]);

        assert_eq!(
            world
                .snake(0)
                .map(|snake| (snake.heading, snake.cells().collect())),
            Some((Direction::Up, points(&[(5, 4), (5, 5), (4, 5)]))),
            "step turns the head, and the body follows"
        );
    }

    #[test]
    fn wall() {
        let mut world = world(vec![snake(&[(9, 5), (8, 5)], Direction::Right)]);
        step(&mut world, &[Turn::Straight]);

        assert_eq!(
            world.slots,
            [Slot::Respawning(4)],
            "step kills a snake that leaves the board, starting its respawn wait"
        );
    }

    #[test]
    fn body() {
        let mut world = world(vec![
            snake(&[(5, 5), (4, 5)], Direction::Right),
            snake(&[(6, 4), (6, 5), (6, 6)], Direction::Up),
        ]);
        step(&mut world, &[Turn::Straight, Turn::Straight]);

        assert_eq!(
            alive(&world),
            [false, true],
            "step kills a snake that moves into another's body"
        );
    }

    #[test]
    fn tail() {
        let mut world = world(vec![
            snake(&[(5, 5), (4, 5)], Direction::Right),
            snake(&[(6, 3), (6, 4), (6, 5)], Direction::Up),
        ]);
        step(&mut world, &[Turn::Straight, Turn::Straight]);

        assert_eq!(
            alive(&world),
            [false, true],
            "step kills a snake that moves into a tail, even as it leaves"
        );
    }

    #[test]
    fn head_on() {
        let mut world = world(vec![
            snake(&[(4, 5), (3, 5)], Direction::Right),
            snake(&[(6, 5), (7, 5)], Direction::Left),
        ]);
        step(&mut world, &[Turn::Straight, Turn::Straight]);

        assert_eq!(
            alive(&world),
            [false, false],
            "step kills both snakes when their heads meet"
        );
    }

    #[test]
    fn swap() {
        let mut world = world(vec![
            snake(&[(4, 5), (3, 5)], Direction::Right),
            snake(&[(5, 5), (6, 5)], Direction::Left),
        ]);
        step(&mut world, &[Turn::Straight, Turn::Straight]);

        assert_eq!(
            alive(&world),
            [false, false],
            "step kills both snakes when their heads swap places"
        );
    }

    #[test]
    fn eats() {
        let mut world = world(vec![snake(&[(5, 5), (4, 5)], Direction::Right)]);
        world.food = points(&[(6, 5)]);
        step(&mut world, &[Turn::Straight]);

        assert_eq!(
            world.snake(0),
            Some(&snake(&[(6, 5), (5, 5), (4, 5)], Direction::Right)),
            "step grows a snake that eats"
        );
        assert_eq!(
            world.food,
            points(&[(0, 0)]),
            "step replaces eaten food on a free cell"
        );
    }

    #[test]
    fn starves() {
        let mut hungry = snake(&[(1, 5), (0, 5)], Direction::Right);
        hungry.hunger = 148;
        let mut world = world(vec![hungry]);
        step(&mut world, &[Turn::Straight]);
        let survives = alive(&world);
        step(&mut world, &[Turn::Straight]);

        assert_eq!(
            [survives, alive(&world)],
            [[true], [false]],
            "step kills a snake on its 150th move without eating"
        );
    }
}
