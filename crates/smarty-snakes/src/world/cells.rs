use super::{World, YOU};
use crate::cell::Cell;

impl World {
    /// The board as one cell code per cell, row by row.
    pub(crate) fn cells(&self) -> Box<[u8]> {
        let mut cells = vec![u8::from(Cell::Empty); self.board.area()].into_boxed_slice();
        let mut paint = |point, cell: Cell| {
            if let Some(code) = self.board.index(point).and_then(|i| cells.get_mut(i)) {
                *code = cell.into();
            }
        };

        for &food in &self.food {
            paint(food, Cell::Food);
        }
        for (index, slot) in self.slots.iter().enumerate() {
            let Some(snake) = slot.alive() else { continue };
            let (body, head) = if index == YOU {
                (Cell::You, Cell::YourHead)
            } else {
                (Cell::Clone, Cell::CloneHead)
            };

            for &point in &snake.body {
                paint(point, body);
            }
            paint(snake.head, head);
        }

        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{points, snake};
    use crate::world::{Board, Direction, Slot};

    #[test]
    fn cells() {
        let world = World {
            board: Board::new(9, 9).unwrap(),
            slots: vec![
                Slot::Alive(snake(&[(1, 0), (0, 0)], Direction::Right)),
                Slot::Alive(snake(&[(3, 0), (4, 0)], Direction::Left)),
            ],
            food: points(&[(6, 0)]),
        };
        let cells = world.cells();

        assert_eq!(cells.len(), 81, "cells has one code per cell");
        assert_eq!(
            cells.get(..9),
            Some([2, 3, 0, 5, 4, 0, 1, 0, 0].as_slice()),
            "cells marks food, and each snake's head and body"
        );
    }
}
