use std::ops::Range;

use super::point::Point;
use crate::tuning::{CELLS_PER_SNAKE, MIN_SNAKES, SPAWN_MARGIN};

/// The board's size in cells. Its edges are walls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Size {
    width: i32,
    height: i32,
}

impl Size {
    ///
    /// The shortest side a board can have.
    ///
    /// A snake's head needs [`SPAWN_MARGIN`] cells between
    /// it and each edge. So a side must fit that many
    /// cells, then the head, then that many cells again.
    ///
    const MIN_SIDE: i32 = 2 * SPAWN_MARGIN + 1;

    ///
    /// The size of a board `width` by `height` cells.
    ///
    /// Returns `None` if it's too small for a snake to
    /// spawn on.
    ///
    pub(crate) fn new(width: u16, height: u16) -> Option<Self> {
        let width = i32::from(width);
        let height = i32::from(height);
        if width < Self::MIN_SIDE || height < Self::MIN_SIDE {
            return None;
        }

        Some(Self { width, height })
    }

    /// Whether `point` is on the board rather than in a wall
    pub(crate) const fn contains(self, point: Point) -> bool {
        0 <= point.x && point.x < self.width && 0 <= point.y && point.y < self.height
    }

    /// Every cell, row by row
    pub(crate) fn points(self) -> impl Iterator<Item = Point> {
        grid(0..self.width, 0..self.height)
    }

    /// Where `point` falls in a row-by-row list of cells
    pub(crate) fn index(self, point: Point) -> Option<usize> {
        if !self.contains(point) {
            return None;
        }

        usize::try_from(point.y * self.width + point.x).ok()
    }

    /// Cells along the longer edge, the farthest any straight line can run
    pub(crate) const fn longest_side(self) -> i32 {
        if self.width > self.height {
            self.width
        } else {
            self.height
        }
    }

    /// The number of cells
    pub(crate) fn area(self) -> usize {
        usize::try_from(self.width * self.height).unwrap_or(0)
    }

    ///
    /// How many snakes play on a board this size, counting the player's.
    ///
    /// Rounded to the nearest snake.
    ///
    pub(crate) fn snake_count(self) -> usize {
        let area = self.width * self.height;
        let count = (area + CELLS_PER_SNAKE / 2) / CELLS_PER_SNAKE;

        usize::try_from(count).unwrap_or(0).max(MIN_SNAKES)
    }

    /// Cells where a snake's head may spawn, away from the walls
    pub(crate) fn spawn_points(self) -> impl Iterator<Item = Point> {
        grid(
            SPAWN_MARGIN..self.width - SPAWN_MARGIN,
            SPAWN_MARGIN..self.height - SPAWN_MARGIN,
        )
    }
}

/// Every point with `x` in `xs` and `y` in `ys`, row by row
fn grid(xs: Range<i32>, ys: Range<i32>) -> impl Iterator<Item = Point> {
    ys.flat_map(move |y| xs.clone().map(move |x| Point { x, y }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_small() {
        assert_eq!(
            [Size::new(8, 20), Size::new(20, 8)],
            [None, None],
            "new rejects boards without room to spawn"
        );
    }

    #[test]
    fn contains() {
        let size = Size::new(10, 9).unwrap();
        let points = [(0, 0), (9, 8), (-1, 0), (0, -1), (10, 0), (0, 9)]
            .map(|(x, y)| size.contains(Point { x, y }));

        assert_eq!(
            points,
            [true, true, false, false, false, false],
            "contains is true only for points on the board"
        );
    }

    #[test]
    fn index() {
        let size = Size::new(10, 9).unwrap();

        assert_eq!(
            [Point { x: 3, y: 2 }, Point { x: 10, y: 0 }].map(|point| size.index(point)),
            [Some(23), None],
            "index counts row by row, and skips points off the board"
        );
    }

    #[test]
    fn snake_count() {
        let counts = [(20, 28), (48, 30), (9, 9)]
            .map(|(width, height)| Size::new(width, height).unwrap().snake_count());

        assert_eq!(
            counts,
            [3, 7, 2],
            "snake_count gives one snake per 200 cells, and at least two"
        );
    }

    #[test]
    fn spawn_points() {
        let size = Size::new(10, 9).unwrap();
        let points: Vec<_> = size.spawn_points().collect();

        assert_eq!(
            points,
            [Point { x: 4, y: 4 }, Point { x: 5, y: 4 }],
            "spawn_points keeps heads 4 cells from every edge"
        );
    }
}
