use rand::Rng;
use rand::seq::IteratorRandom;

use super::{Direction, Point, Snake, SnakeState, SnakeTail, World};
use crate::tuning::SPAWN_LENGTH;

impl World {
    ///
    /// Counts down each dead snake's wait, respawning
    /// those whose wait is over
    ///
    pub(crate) fn count_down(&mut self, rng: &mut impl Rng) {
        for index in 0..self.snakes.len() {
            let Some(SnakeState::Respawning(moves)) = self.snakes.get_mut(index) else {
                continue;
            };
            *moves = moves.saturating_sub(1);
            if *moves == 0 {
                self.respawn(index, rng);
            }
        }
    }

    ///
    /// Brings dead snake `index` back as a new snake, in a
    /// random spot where it fits.
    ///
    /// If there's no such spot, it waits to try again.
    ///
    pub(crate) fn respawn(&mut self, index: usize, rng: &mut impl Rng) {
        let spots = self
            .size
            .spawn_points()
            .flat_map(each_heading)
            .filter(|&(head, heading)| self.snake_fits(head, heading));
        let Some((head, heading)) = spots.choose(rng) else {
            self.set_snake(index, SnakeState::Respawning(0));
            return;
        };

        self.spawn(index, head, heading);
    }

    ///
    /// Whether a new snake fits with its head on `head`,
    /// facing `heading`.
    ///
    /// Its own cells must be free, and as many cells
    /// ahead, so it never spawns into a crash.
    ///
    fn snake_fits(&self, head: Point, heading: Direction) -> bool {
        let tail_end = 1 - SPAWN_LENGTH;
        let ahead = SPAWN_LENGTH;

        (tail_end..=ahead).all(|steps| self.is_free(head.step(heading, steps)))
    }

    ///
    /// Makes snake `index` a new snake, with its head on
    /// `head` and its tail straight behind
    ///
    fn spawn(&mut self, index: usize, head: Point, heading: Direction) {
        let behind = heading.opposite();
        let tail = SnakeTail {
            snake: index,
            toward_head: heading,
        };
        for steps in 1..SPAWN_LENGTH {
            self.set_tail(head.step(behind, steps), Some(tail));
        }

        let snake = Snake {
            head,
            heading,
            tail_end: head.step(behind, SPAWN_LENGTH - 1),
        };
        self.set_snake(index, SnakeState::Alive(snake));
    }
}

/// `head` paired with each direction a snake there could face
fn each_heading(head: Point) -> [(Point, Direction); 4] {
    Direction::ALL.map(|heading| (head, heading))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{self, SnakeCells, points, seeded, snake, snake_cells};

    fn world(snakes: Vec<SnakeCells>, waits: &[u32]) -> World {
        let mut world = testing::world(10, 10, snakes);
        world
            .snakes
            .extend(waits.iter().copied().map(SnakeState::Respawning));

        world
    }

    #[test]
    fn respawn() {
        let mut world = world(vec![], &[0]);
        world.respawn(0, &mut seeded(1));
        let spawned = *world.snake(0).unwrap();
        let behind = spawned.heading.opposite();

        assert_eq!(
            snake_cells(&world, 0),
            [0, 1, 2].map(|steps| spawned.head.step(behind, steps)),
            "respawn spawns a snake 3 cells long, with its tail straight behind"
        );
        assert!(
            world.size.spawn_points().any(|point| point == spawned.head),
            "respawn spawns the head away from the walls"
        );
    }

    #[test]
    fn respawn_fits() {
        let blocker = snake(&[(7, 4), (8, 4), (9, 4)], Direction::West);
        let mut world = world(vec![blocker], &[0]);
        world.food = points(&[(4, 5), (5, 5)]);
        world.respawn(1, &mut seeded(1));

        assert_eq!(
            world.snake(1).map(|snake| (snake.head, snake.heading)),
            Some((Point { x: 4, y: 4 }, Direction::West)),
            "respawn takes the only spot where snakes and food leave its cells and path ahead free"
        );
    }

    #[test]
    fn respawn_full() {
        let blocker = snake(
            &[
                (4, 2),
                (4, 3),
                (4, 4),
                (4, 5),
                (5, 5),
                (5, 4),
                (5, 3),
                (5, 2),
            ],
            Direction::North,
        );
        let mut world = world(vec![blocker], &[1]);
        world.respawn(1, &mut seeded(1));

        assert_eq!(
            world.snakes.get(1),
            Some(&SnakeState::Respawning(0)),
            "respawn waits when there's nowhere to spawn"
        );
    }

    #[test]
    fn count_down() {
        let mut world = world(vec![], &[4]);
        let mut rng = seeded(1);
        let waits: Vec<_> = (0..4)
            .map(|_| {
                world.count_down(&mut rng);
                world.snake(0).is_some()
            })
            .collect();

        assert_eq!(
            waits,
            [false, false, false, true],
            "count_down respawns a snake on the 4th move"
        );
    }
}
