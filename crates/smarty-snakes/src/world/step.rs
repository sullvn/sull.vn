use rand::Rng;

use super::{Move, World};

impl World {
    ///
    /// Makes every move in `moves` at once.
    ///
    /// A snake dies if its new head is off the board, on a
    /// cell any snake covered before the move, or on
    /// another snake's new head. So moving into a tail
    /// that's about to leave is deadly, and so is swapping
    /// places head to head.
    ///
    /// The dead leave at once and start counting down to
    /// respawn.
    ///
    pub(crate) fn step(&mut self, moves: &[Move], rng: &mut impl Rng) {
        // Every move is judged against the board as it
        // stands, before any snake moves
        let (crashed, survivors): (Vec<_>, Vec<_>) = moves
            .iter()
            .copied()
            .partition(|&step| self.crashes(step, moves));

        for step in crashed {
            self.kill(step.snake);
        }
        for step in survivors {
            self.advance(step);
        }

        self.add_food(rng);
    }

    ///
    /// Whether making `step` kills a snake, when every
    /// snake makes its move in `moves` at once
    ///
    fn crashes(&self, step: Move, moves: &[Move]) -> bool {
        let same_head = moves.iter().filter(|other| other.head == step.head).count();

        self.is_blocked(step.head) || same_head > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{self, SnakeCells, points, seeded, snake, snake_cells};
    use crate::turn::Turn;
    use crate::world::{Direction, Point, SnakeState};

    fn world(snakes: Vec<SnakeCells>) -> World {
        testing::world(10, 10, snakes)
    }

    fn step(world: &mut World, turns: &[Turn]) {
        let moves: Vec<_> = world
            .living_indexed()
            .zip(turns)
            .map(|((index, snake), &turn)| snake.next(index, turn))
            .collect();
        world.step(&moves, &mut seeded(1));
    }

    fn alive(world: &World) -> Vec<bool> {
        world
            .snakes
            .iter()
            .map(|state| state.alive().is_some())
            .collect()
    }

    #[test]
    fn moves() {
        let mut world = world(vec![snake(&[(5, 5), (4, 5), (3, 5)], Direction::East)]);
        step(&mut world, &[Turn::Left]);

        assert_eq!(
            (
                world.snake(0).map(|snake| snake.heading),
                snake_cells(&world, 0)
            ),
            (Some(Direction::North), points(&[(5, 4), (5, 5), (4, 5)])),
            "step turns the head, and the tail follows"
        );
    }

    #[test]
    fn wall() {
        let mut world = world(vec![snake(&[(9, 5), (8, 5)], Direction::East)]);
        step(&mut world, &[Turn::Straight]);

        assert_eq!(
            world.snakes,
            [SnakeState::Respawning(4)],
            "step kills a snake that leaves the board, starting its respawn wait"
        );
    }

    #[test]
    fn body() {
        let mut world = world(vec![
            snake(&[(5, 5), (4, 5)], Direction::East),
            snake(&[(6, 4), (6, 5), (6, 6)], Direction::North),
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
            snake(&[(5, 5), (4, 5)], Direction::East),
            snake(&[(6, 3), (6, 4), (6, 5)], Direction::North),
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
            snake(&[(9, 5), (8, 5), (7, 5)], Direction::East),
            snake(&[(6, 5), (6, 6)], Direction::North),
        ]);
        step(&mut world, &[Turn::Straight, Turn::Right]);

        assert_eq!(
            alive(&world),
            [false, false],
            "step kills a snake that moves into the cells of one that dies in the same move"
        );
        assert_eq!(
            [(9, 5), (8, 5), (7, 5), (6, 5), (6, 6)].map(|(x, y)| world.is_blocked(Point { x, y })),
            [false; 5],
            "step clears every cell of the snakes that die"
        );
    }

    #[test]
    fn head_on() {
        let mut world = world(vec![
            snake(&[(4, 5), (3, 5)], Direction::East),
            snake(&[(6, 5), (7, 5)], Direction::West),
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
            snake(&[(4, 5), (3, 5)], Direction::East),
            snake(&[(5, 5), (6, 5)], Direction::West),
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
        let mut world = world(vec![snake(&[(5, 5), (4, 5)], Direction::East)]);
        world.food = points(&[(6, 5)]);
        step(&mut world, &[Turn::Straight]);

        assert_eq!(
            snake_cells(&world, 0),
            points(&[(6, 5), (5, 5), (4, 5)]),
            "step grows a snake that eats"
        );
        assert_eq!(world.food.len(), 1, "step replaces eaten food");
        assert!(
            world.food.iter().all(|&food| !world.is_blocked(food)),
            "step puts the new food on a free cell"
        );
    }
}
