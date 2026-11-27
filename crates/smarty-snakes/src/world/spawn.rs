use super::{Direction, Slot, Snake, World};
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
    /// Puts a new snake in slot `index`, in a random
    /// clear spot.
    ///
    /// The cells it covers and the cells ahead of it are
    /// free, so it never spawns into a crash. If there's
    /// no such spot, the slot waits to try again.
    ///
    pub(crate) fn respawn(&mut self, index: usize, random: &mut impl FnMut() -> f64) {
        let taken = self.taken();
        let spots: Vec<_> = self
            .board
            .spawn_points()
            .flat_map(|head| Direction::ALL.map(|heading| (head, heading)))
            .filter(|&(head, heading)| {
                (1 - SPAWN_LENGTH..=SPAWN_LENGTH)
                    .all(|steps| !taken.contains(&head.step(heading, steps)))
            })
            .collect();
        let slot = pick(random, &spots).map_or(Slot::Respawning(0), |&(head, heading)| {
            Slot::Alive(Snake::spawn(head, heading))
        });

        if let Some(old) = self.slots.get_mut(index) {
            *old = slot;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{points, scripted, snake};
    use crate::world::{Board, Point};

    fn world(slots: Vec<Slot>) -> World {
        World {
            board: Board::new(10, 10).unwrap(),
            slots,
            food: Vec::new(),
        }
    }

    #[test]
    fn respawn() {
        let mut world = world(vec![Slot::Respawning(0)]);
        world.respawn(0, &mut scripted(&[0.0]));

        assert_eq!(
            world.slots,
            [Slot::Alive(snake(&[(4, 4), (4, 5), (4, 6)], Direction::Up))],
            "respawn spawns a snake away from the walls"
        );
    }

    #[test]
    fn respawn_clear() {
        let blocker = Slot::Alive(snake(&[(4, 1), (4, 2), (4, 3)], Direction::Down));
        let mut world = world(vec![blocker, Slot::Respawning(0)]);
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
        let blocker = Slot::Alive(snake(
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
        ));
        let mut world = world(vec![blocker, Slot::Alive(snake(&[(0, 0)], Direction::Up))]);
        world.respawn(1, &mut scripted(&[0.0]));

        assert_eq!(
            world.slots.get(1),
            Some(&Slot::Respawning(0)),
            "respawn waits when there's nowhere to spawn"
        );
    }

    #[test]
    fn count_down() {
        let mut world = world(vec![Slot::Respawning(4)]);
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
