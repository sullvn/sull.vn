use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::world::{Direction, Point, Snake};

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
