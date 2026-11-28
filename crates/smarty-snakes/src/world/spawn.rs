use super::{Direction, Point, Snake, SnakeState, Tail, World};
use crate::random::pick;
use crate::tuning::SPAWN_LENGTH;

impl World {
    ///
    /// Counts down each dead snake's wait, respawning
    /// those whose wait is over.
    ///
    pub(crate) fn count_down(&mut self, random: &mut impl FnMut() -> f64) {
        for index in 0..self.snakes.len() {
            let Some(SnakeState::Respawning(moves)) = self.snakes.get_mut(index) else {
                continue;
            };
            *moves = moves.saturating_sub(1);
            if *moves == 0 {
                self.respawn(index, random);
            }
        }
    }

    ///
    /// Brings dead snake `index` back as a new snake, in a
    /// random clear spot.
    ///
    /// The cells it covers and the cells ahead of it are
    /// free, so it never spawns into a crash. If there's
    /// no such spot, it waits to try again.
    ///
    pub(crate) fn respawn(&mut self, index: usize, random: &mut impl FnMut() -> f64) {
        let spots: Vec<_> = self
            .size
            .spawn_points()
            .flat_map(|head| Direction::ALL.map(|heading| (head, heading)))
            .filter(|&(head, heading)| {
                (1 - SPAWN_LENGTH..=SPAWN_LENGTH).all(|steps| self.free(head.step(heading, steps)))
            })
            .collect();
        let Some(&(head, heading)) = pick(random, &spots) else {
            self.set_snake(index, SnakeState::Respawning(0));
            return;
        };

        self.spawn(index, head, heading);
    }

    ///
    /// Makes snake `index` a new snake, with its head on
    /// `head` and its tail straight behind.
    ///
    fn spawn(&mut self, index: usize, head: Point, heading: Direction) {
        let behind = heading.opposite();
        let tail = Tail {
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
            hunger: 0,
        };
        self.set_snake(index, SnakeState::Alive(snake));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{self, SnakeCells, points, scripted, snake, snake_cells};

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
        world.respawn(0, &mut scripted(&[0.0]));

        assert_eq!(
            (
                world.snake(0).map(|snake| snake.heading),
                snake_cells(&world, 0)
            ),
            (Some(Direction::North), points(&[(4, 4), (4, 5), (4, 6)])),
            "respawn spawns a snake 3 cells long, away from the walls"
        );
    }

    #[test]
    fn respawn_clear() {
        let blocker = snake(&[(4, 1), (4, 2), (4, 3)], Direction::South);
        let mut world = world(vec![blocker], &[0]);
        world.food = points(&[(6, 4)]);
        world.respawn(1, &mut scripted(&[0.0]));

        assert_eq!(
            world.snake(1).map(|snake| (snake.head, snake.heading)),
            Some((Point { x: 5, y: 4 }, Direction::North)),
            "respawn skips spots where snakes or food cover its cells or path ahead"
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
        world.respawn(1, &mut scripted(&[0.0]));

        assert_eq!(
            world.snakes.get(1),
            Some(&SnakeState::Respawning(0)),
            "respawn waits when there's nowhere to spawn"
        );
    }

    #[test]
    fn count_down() {
        let mut world = world(vec![], &[4]);
        let mut random = scripted(&[0.0]);
        let waits: Vec<_> = (0..4)
            .map(|_| {
                world.count_down(&mut random);
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
