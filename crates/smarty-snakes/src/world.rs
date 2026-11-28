use crate::tuning::STARVATION_MOVES;
use crate::turn::Turn;

mod cells;
mod direction;
mod food;
mod point;
mod size;
mod snake;
mod spawn;

pub(crate) use direction::Direction;
pub(crate) use point::Point;
pub(crate) use size::Size;
pub(crate) use snake::{Snake, Tail};

use snake::Move;

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
    /// The board's width and height in cells.
    pub(crate) size: Size,

    /// Each snake's head, heading and hunger, or its wait to respawn.
    pub(crate) slots: Vec<Slot>,

    /// The piece of tail on each cell, if any, row by row.
    tails: Box<[Option<Tail>]>,

    /// Where the food is.
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
    pub(crate) fn new(size: Size, random: &mut impl FnMut() -> f64) -> Self {
        let mut world = Self::empty(size);
        world.slots = vec![Slot::Respawning(0); size.snake_count()];
        for index in 0..world.slots.len() {
            world.respawn(index, random);
        }
        world.add_food(random);

        world
    }

    /// A board with no snakes or food.
    pub(crate) fn empty(size: Size) -> Self {
        Self {
            size,
            slots: Vec::new(),
            tails: vec![None; size.area()].into_boxed_slice(),
            food: Vec::new(),
        }
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
        !self.size.contains(point)
            || self.tail(point).is_some()
            || self.snakes().any(|snake| snake.head == point)
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
        let moves: Vec<_> = self
            .slots
            .iter()
            .zip(turns)
            .enumerate()
            .filter_map(|(index, (slot, &turn))| Some((index, slot.alive()?.next(turn?))))
            .collect();
        let (crashed, survivors): (Vec<_>, Vec<_>) = moves
            .iter()
            .copied()
            .partition(|&(_, step)| self.crashes(step, &moves));

        for (index, _) in crashed {
            self.kill(index);
        }
        for (index, step) in survivors {
            self.advance(index, step);
            if self.starved(index) {
                self.kill(index);
            }
        }

        self.add_food(random);
    }

    ///
    /// Whether making `step` kills a snake, when every
    /// snake makes its move in `moves` at once.
    ///
    fn crashes(&self, step: Move, moves: &[(usize, Move)]) -> bool {
        let same_head = moves
            .iter()
            .filter(|(_, other)| other.head == step.head)
            .count();

        self.blocked(step.head) || same_head > 1
    }

    /// Whether the snake in slot `index` has gone too long without eating.
    fn starved(&self, index: usize) -> bool {
        self.snake(index)
            .is_some_and(|snake| snake.hunger >= STARVATION_MOVES)
    }

    /// Whether `point` is on the board with no snake or food on it.
    fn free(&self, point: Point) -> bool {
        !self.blocked(point) && !self.food.contains(&point)
    }

    /// Puts `slot` in place of slot `index`.
    fn set_slot(&mut self, index: usize, slot: Slot) {
        if let Some(old) = self.slots.get_mut(index) {
            *old = slot;
        }
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
    use crate::testing::{self, SnakeCells, points, scripted, snake, snake_cells};

    fn world(snakes: Vec<SnakeCells>) -> World {
        testing::world(10, 10, snakes)
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
        let world = World::new(Size::new(20, 28).unwrap(), &mut scripted(&[0.3, 0.7]));

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
            (
                world.snake(0).map(|snake| snake.heading),
                snake_cells(&world, 0)
            ),
            (Some(Direction::Up), points(&[(5, 4), (5, 5), (4, 5)])),
            "step turns the head, and the tail follows"
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
    fn dead_snakes_cells() {
        let mut world = world(vec![
            snake(&[(9, 5), (8, 5), (7, 5)], Direction::Right),
            snake(&[(6, 5), (6, 6)], Direction::Up),
        ]);
        step(&mut world, &[Turn::Straight, Turn::Right]);

        assert_eq!(
            alive(&world),
            [false, false],
            "step kills a snake that moves into the cells of one that dies in the same move"
        );
        assert_eq!(
            [(9, 5), (8, 5), (7, 5), (6, 5), (6, 6)].map(|(x, y)| world.blocked(Point { x, y })),
            [false; 5],
            "step clears every cell of the snakes that die"
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
            snake_cells(&world, 0),
            points(&[(6, 5), (5, 5), (4, 5)]),
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
        let mut world = world(vec![snake(&[(1, 5), (0, 5)], Direction::Right)]);
        if let Some(Slot::Alive(hungry)) = world.slots.first_mut() {
            hungry.hunger = 148;
        }
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
