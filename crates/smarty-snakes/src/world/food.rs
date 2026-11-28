use super::World;
use crate::random::pick;

impl World {
    ///
    /// Puts food on random free cells until there's one
    /// piece per snake.
    ///
    pub(crate) fn add_food(&mut self, random: &mut impl FnMut() -> f64) {
        while self.food.len() < self.slots.len() {
            let free: Vec<_> = self
                .size
                .points()
                .filter(|&point| self.free(point))
                .collect();
            let Some(&food) = pick(random, &free) else {
                return;
            };

            self.food.push(food);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{points, scripted, snake, world};
    use crate::world::{Direction, Slot};

    #[test]
    fn add_food() {
        let mut world = world(10, 10, vec![snake(&[(1, 0), (2, 0)], Direction::West)]);
        world.slots.push(Slot::Respawning(1));
        world.food = points(&[(0, 0)]);
        world.add_food(&mut scripted(&[0.0]));

        assert_eq!(
            world.food,
            points(&[(0, 0), (3, 0)]),
            "add_food adds one piece per snake, on cells without snakes or food"
        );
    }

    #[test]
    fn add_food_full() {
        let mut world = world(10, 10, vec![]);
        world.slots = vec![Slot::Respawning(1); 101];
        world.food = world.size.points().skip(1).collect();
        world.add_food(&mut scripted(&[0.0]));

        assert_eq!(
            world.food.len(),
            100,
            "add_food fills the last free cell, then stops"
        );
    }
}
