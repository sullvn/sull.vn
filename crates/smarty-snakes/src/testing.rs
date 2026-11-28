use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::controls::Input;
use crate::random::pick;
use crate::turn::Turn;
use crate::world::{Direction, Point, Size, Slot, Snake, Tail, World, YOU};

/// A snake to put in a test world.
#[derive(Debug)]
pub(crate) struct SnakeCells {
    /// The cells it covers, from head to tail.
    cells: Vec<Point>,
    heading: Direction,
}

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
        Direction::North => Input::Up,
        Direction::East => Input::Right,
        Direction::South => Input::Down,
        Direction::West => Input::Left,
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
pub(crate) fn snake(cells: &[(i32, i32)], heading: Direction) -> SnakeCells {
    SnakeCells {
        cells: points(cells),
        heading,
    }
}

///
/// A world `width` by `height` cells with no food,
/// holding `snakes` in slot order.
///
pub(crate) fn world(width: u16, height: u16, snakes: Vec<SnakeCells>) -> World {
    let mut world = World::empty(Size::new(width, height).unwrap());

    for (index, SnakeCells { cells, heading }) in snakes.into_iter().enumerate() {
        for pair in cells.windows(2) {
            let tail = Tail {
                snake: index,
                toward_head: toward(pair[1], pair[0]),
            };
            world.set_tail(pair[1], Some(tail));
        }
        world.slots.push(Slot::Alive(Snake {
            head: cells[0],
            heading,
            tail_end: *cells.last().unwrap(),
            hunger: 0,
        }));
    }

    world
}

/// The cells of the snake in slot `index`, from head to tail.
pub(crate) fn snake_cells(world: &World, index: usize) -> Vec<Point> {
    let snake = world.snake(index).unwrap();
    let mut cells = Vec::new();
    let mut point = snake.tail_end;
    while let Some(tail) = world.tail(point) {
        cells.push(point);
        point = point.step(tail.toward_head, 1);
    }
    assert_eq!(point, snake.head, "a snake's tail leads to its head");
    cells.push(point);
    cells.reverse();

    cells
}

/// The direction from `from` to the cell next to it, `to`.
fn toward(from: Point, to: Point) -> Direction {
    Direction::ALL
        .into_iter()
        .find(|&direction| from.step(direction, 1) == to)
        .unwrap()
}
