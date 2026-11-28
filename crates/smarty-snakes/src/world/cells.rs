use super::{SnakeTail, World, YOU};
use crate::cell::Cell;

impl World {
    /// The board as one cell code per cell, row by row.
    pub(crate) fn cells(&self) -> Box<[u8]> {
        let mut cells: Box<[u8]> = self
            .tails
            .iter()
            .map(|&tail| tail_cell(tail).into())
            .collect();
        let food = self.food.iter().map(|&food| (food, Cell::Food));
        let heads = self
            .snakes
            .iter()
            .enumerate()
            .filter_map(|(index, state)| Some((state.alive()?.head, head_cell(index))));

        for (point, cell) in food.chain(heads) {
            let code = self
                .size
                .index(point)
                .and_then(|index| cells.get_mut(index));
            if let Some(code) = code {
                *code = cell.into();
            }
        }

        cells
    }
}

/// The cell for a piece of tail, or for no tail.
const fn tail_cell(tail: Option<SnakeTail>) -> Cell {
    match tail {
        None => Cell::Empty,
        Some(SnakeTail { snake: YOU, .. }) => Cell::You,
        Some(_) => Cell::Clone,
    }
}

/// The cell for the head of snake `index`.
const fn head_cell(index: usize) -> Cell {
    if index == YOU {
        Cell::YourHead
    } else {
        Cell::CloneHead
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{points, snake, world};
    use crate::world::Direction;

    #[test]
    fn cells() {
        let mut world = world(
            9,
            9,
            vec![
                snake(&[(1, 0), (0, 0)], Direction::East),
                snake(&[(3, 0), (4, 0)], Direction::West),
            ],
        );
        world.food = points(&[(6, 0)]);
        let cells = world.cells();

        assert_eq!(cells.len(), 81, "cells has one code per cell");
        assert_eq!(
            cells.get(..9),
            Some([2, 3, 0, 5, 4, 0, 1, 0, 0].as_slice()),
            "cells marks food, and each snake's head and tail"
        );
    }
}
