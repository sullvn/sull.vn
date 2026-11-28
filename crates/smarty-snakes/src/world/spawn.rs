use super::{Direction, Point, Slot, Snake, Tail, World};
use crate::random::pick;
use crate::tuning::SPAWN_LENGTH;

impl World {
    ///
    /// Counts down each dead snake's wait, respawning
    /// those whose wait is over.
    ///
    pub(crate) fn count_down(&mut self, random: &mut impl FnMut() -> f64) {
        for index in 0..self.slots.len() {
            let Some(Slot::Respawning(moves)) = self.slots.get_mut(index) else {
                continue;
            };
            *moves = moves.saturating_sub(1);
            if *moves == 0 {
                self.respawn(index, random);
            }
        }
    }

    ///
    /// Puts a new snake in the empty slot `index`, in a
    /// random clear spot.
    ///
    /// The cells it covers and the cells ahead of it are
    /// free, so it never spawns into a crash. If there's
    /// no such spot, the slot waits to try again.
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
            self.set_slot(index, Slot::Respawning(0));
            return;
        };

        self.spawn(index, head, heading);
    }

    ///
    /// Puts a new snake in slot `index`, with its head on
    /// `head` and its tail straight behind.
    ///
    fn spawn(&mut self, index: usize, head: Point, heading: Direction) {
        let behind = heading.right().right();
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
        self.set_slot(index, Slot::Alive(snake));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{self, SnakeCells, points, scripted, snake, snake_cells};

    fn world(snakes: Vec<SnakeCells>, waits: &[u32]) -> World {
        let mut world = testing::world(10, 10, snakes);
        world
            .slots
            .extend(waits.iter().copied().map(Slot::Respawning));

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
            (Some(Direction::Up), points(&[(4, 4), (4, 5), (4, 6)])),
            "respawn spawns a snake 3 cells long, away from the walls"
        );
    }

    #[test]
    fn respawn_clear() {
        let blocker = snake(&[(4, 1), (4, 2), (4, 3)], Direction::Down);
        let mut world = world(vec![blocker], &[0]);
        world.food = points(&[(6, 4)]);
        world.respawn(1, &mut scripted(&[0.0]));

        assert_eq!(
            world.snake(1).map(|snake| (snake.head, snake.heading)),
            Some((Point { x: 5, y: 4 }, Direction::Up)),
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
            Direction::Up,
        );
        let mut world = world(vec![blocker], &[1]);
        world.respawn(1, &mut scripted(&[0.0]));

        assert_eq!(
            world.slots.get(1),
            Some(&Slot::Respawning(0)),
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
