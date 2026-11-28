use crate::tuning::SENSE_RANGE;
use crate::turn::Turn;
use crate::world::{Point, Snake, World};

///
/// What a snake notices, all relative to its heading.
///
/// 1. Obstacle to the left
/// 2. Obstacle ahead
/// 3. Obstacle to the right
/// 4. Nearest food, cells ahead
/// 5. Nearest food, cells to the right
/// 6. Nearest food, whether ahead or behind
/// 7. Nearest food, whether right or left
/// 8. Nearest other snake's head, cells ahead
/// 9. Nearest other snake's head, cells to the right
///
/// Obstacles are walls or any snake's body, and range
/// from 1 when right next to the head toward -1 when
/// far away. Cell counts are scaled by [`SENSE_RANGE`]
/// to -1 to 1, and directions are 1, 0 or -1.
///
pub(crate) type Senses = [f64; 9];

/// What `snake` notices in `world`.
pub(crate) fn sense(world: &World, snake: &Snake) -> Senses {
    let [left, ahead, right] = Turn::ALL.map(|turn| obstacle(world, snake, turn));
    let (food_ahead, food_right) = nearest(snake, world.food.iter().copied());
    let other_heads = world
        .living()
        .map(|other| other.head)
        .filter(|&head| head != snake.head);
    let (other_ahead, other_right) = nearest(snake, other_heads);

    [
        left,
        ahead,
        right,
        scale(food_ahead),
        scale(food_right),
        f64::from(food_ahead.signum()),
        f64::from(food_right.signum()),
        scale(other_ahead),
        scale(other_right),
    ]
}

///
/// Closeness of the first blocked cell after `turn`.
///
/// `2 / distance - 1` gives 1 for a blocked cell next
/// to the head, 0 with one free cell between, and
/// heads toward -1 as the distance grows.
///
fn obstacle(world: &World, snake: &Snake, turn: Turn) -> f64 {
    let direction = snake.heading.turned(turn);

    (1..=world.size.longest_side())
        .find(|&steps| world.is_blocked(snake.head.step(direction, steps)))
        .map_or(-1.0, |steps| 2.0 / f64::from(steps) - 1.0)
}

///
/// Cells ahead and to the right of the nearest of
/// `points`, or zeros if there are none.
///
fn nearest(snake: &Snake, points: impl Iterator<Item = Point>) -> (i32, i32) {
    points
        .min_by_key(|&point| snake.head.distance(point))
        .map_or((0, 0), |point| snake.head.relative(snake.heading, point))
}

/// Scales a cell count to -1 to 1.
fn scale(cells: i32) -> f64 {
    (f64::from(cells) / SENSE_RANGE).clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{self, SnakeCells, points, snake};
    use crate::world::{Direction, YOU};

    fn world(snakes: Vec<SnakeCells>, food: &[(i32, i32)]) -> World {
        let mut world = testing::world(10, 20, snakes);
        world.food = points(food);

        world
    }

    /// What you notice in `world`.
    fn senses(world: &World) -> Senses {
        sense(world, world.snake(YOU).unwrap())
    }

    #[test]
    fn obstacles() {
        let world = world(
            vec![
                snake(&[(5, 5), (5, 6)], Direction::North),
                snake(&[(4, 5)], Direction::North),
                snake(&[(5, 3)], Direction::North),
            ],
            &[],
        );

        assert_eq!(
            senses(&world).get(..3),
            Some([1.0, 0.0, 2.0 / 5.0 - 1.0].as_slice()),
            "sense finds the closeness of snakes and walls to the left, ahead, and right"
        );
    }

    #[test]
    fn food() {
        let world = world(
            vec![snake(&[(5, 5)], Direction::East)],
            &[(1, 4), (8, 4), (7, 19)],
        );

        assert_eq!(
            senses(&world).get(3..7),
            Some([0.3, -0.1, 1.0, -1.0].as_slice()),
            "sense finds the nearest food, ahead and to the right"
        );
    }

    #[test]
    fn food_far() {
        let world = world(vec![snake(&[(5, 0)], Direction::South)], &[(5, 15)]);

        assert_eq!(
            senses(&world).get(3..7),
            Some([1.0, 0.0, 1.0, 0.0].as_slice()),
            "sense caps food distances at 10 cells"
        );
    }

    #[test]
    fn other_heads() {
        let world = world(
            vec![
                snake(&[(5, 5)], Direction::West),
                snake(&[(5, 7), (5, 8)], Direction::North),
                snake(&[(9, 19)], Direction::North),
            ],
            &[],
        );

        assert_eq!(
            senses(&world).get(7..),
            Some([0.0, -0.2].as_slice()),
            "sense finds the nearest other head, ahead and to the right"
        );
    }

    #[test]
    fn alone() {
        let world = world(vec![snake(&[(5, 5)], Direction::North)], &[]);

        assert_eq!(
            senses(&world).get(3..),
            Some([0.0; 6].as_slice()),
            "sense gives zeros for food and heads that aren't there"
        );
    }
}
