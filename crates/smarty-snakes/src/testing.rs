use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::{Rng, RngExt, SeedableRng};

use crate::controls::Input;
use crate::turn::Turn;
use crate::world::{Direction, PLAYER, Point, Size, Snake, SnakeState, SnakeTail, World};

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
pub(crate) fn food_seeker(world: &World, rng: &mut impl Rng) -> Input {
    let Some(player) = world.snake(PLAYER) else {
        return Input::Up;
    };
    let safe: Vec<_> = [Turn::Straight, Turn::Left, Turn::Right]
        .map(|turn| player.heading.turned(turn))
        .into_iter()
        .filter(|&direction| !world.is_blocked(player.head.step(direction, 1)))
        .collect();
    let food = world
        .food
        .iter()
        .min_by_key(|&&food| player.head.distance(food));
    let closest = safe.iter().min_by_key(|&&direction| {
        food.map_or(0, |&food| player.head.step(direction, 1).distance(food))
    });
    let direction = if rng.random_bool(0.1) {
        safe.iter().choose(rng)
    } else {
        closest
    };

    match direction.copied().unwrap_or(player.heading) {
        Direction::North => Input::Up,
        Direction::East => Input::Right,
        Direction::South => Input::Down,
        Direction::West => Input::Left,
    }
}

/// A source of chance that repeats for each `seed`.
pub(crate) fn seeded(seed: u64) -> SmallRng {
    SmallRng::seed_from_u64(seed)
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
/// holding `snakes` in order.
///
pub(crate) fn world(width: u16, height: u16, snakes: Vec<SnakeCells>) -> World {
    let mut world = World::empty(Size::new(width, height).unwrap());

    for (index, SnakeCells { cells, heading }) in snakes.into_iter().enumerate() {
        for pair in cells.windows(2) {
            let tail = SnakeTail {
                snake: index,
                toward_head: toward(pair[1], pair[0]),
            };
            world.set_tail(pair[1], Some(tail));
        }
        world.snakes.push(SnakeState::Alive(Snake {
            head: cells[0],
            heading,
            tail_end: *cells.last().unwrap(),
            hunger: 0,
        }));
    }

    world
}

/// The cells of snake `index`, from head to tail.
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
