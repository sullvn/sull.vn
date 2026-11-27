use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::controls::Input;
use crate::random::pick;
use crate::turn::Turn;
use crate::world::{Direction, Point, Snake, World, YOU};

///
/// The input from a player who heads for the nearest
/// food and avoids crashing, but 1 time in 10 makes a
/// random safe move instead.
///
pub(crate) fn food_seeker(world: &World, random: &mut impl FnMut() -> f64) -> Input {
    let Some(you) = world.snake(YOU) else {
        return Input::Up;
    };
    let safe: Vec<_> = [Turn::Straight, Turn::Left, Turn::Right]
        .map(|turn| you.heading.turned(turn))
        .into_iter()
        .filter(|&direction| !world.blocked(you.head.step(direction, 1)))
        .collect();
    let food = world
        .food
        .iter()
        .min_by_key(|&&food| you.head.distance(food));
    let closest = safe.iter().min_by_key(|&&direction| {
        food.map_or(0, |&food| you.head.step(direction, 1).distance(food))
    });
    let direction = if random() < 0.1 {
        pick(random, &safe)
    } else {
        closest
    };

    match direction.copied().unwrap_or(you.heading) {
        Direction::Up => Input::Up,
        Direction::Right => Input::Right,
        Direction::Down => Input::Down,
        Direction::Left => Input::Left,
    }
}

/// Random numbers that repeat `values` in order.
pub(crate) fn scripted(values: &[f64]) -> impl FnMut() -> f64 {
    let mut values = values.iter().copied().cycle();
    move || values.next().unwrap_or(0.0)
}

/// Random numbers from 0 to 1 that repeat for each `seed`.
pub(crate) fn seeded(seed: u64) -> impl FnMut() -> f64 {
    let mut rng = StdRng::seed_from_u64(seed);
    move || rng.random()
}

/// Points from `(x, y)` pairs.
pub(crate) fn points(pairs: &[(i32, i32)]) -> Vec<Point> {
    pairs.iter().map(|&(x, y)| Point { x, y }).collect()
}

/// A snake covering `cells`, from head to tail.
pub(crate) fn snake(cells: &[(i32, i32)], heading: Direction) -> Snake {
    let mut cells = points(cells).into_iter();
    let head = cells.next().unwrap();

    Snake {
        head,
        heading,
        body: cells.collect(),
        hunger: 0,
    }
}
