use rand::Rng;
use rand::seq::IteratorRandom;

use super::World;

impl World {
    ///
    /// Puts food on random free cells until there's one
    /// piece per snake
    ///
    pub(crate) fn add_food(&mut self, rng: &mut impl Rng) {
        while self.food.len() < self.snakes.len() {
            let free = self.size.points().filter(|&point| self.is_free(point));
            let Some(food) = free.choose(rng) else {
                return;
            };

            self.food.push(food);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{seeded, snake, world};
    use crate::world::{Direction, Point, SnakeState};

    #[test]
    fn add_food() {
        let mut world = world(10, 10, vec![snake(&[(1, 0), (2, 0)], Direction::West)]);
        world.snakes.push(SnakeState::Respawning(1));
        world.add_food(&mut seeded(1));

        assert_eq!(
            world.food.len(),
            2,
            "add_food adds one piece per snake, alive or not"
        );
    }

    #[test]
    fn add_food_free() {
        let last_free = Point { x: 3, y: 0 };
        let mut world = world(10, 10, vec![snake(&[(1, 0), (2, 0)], Direction::West)]);
        world.snakes.resize(100, SnakeState::Respawning(1));
        world.food = world
            .size
            .points()
            .filter(|&point| world.is_free(point) && point != last_free)
            .collect();
        world.add_food(&mut seeded(1));

        assert_eq!(
            (world.food.len(), world.food.last()),
            (98, Some(&last_free)),
            "add_food only fills cells without snakes or food, then stops"
        );
    }
}
