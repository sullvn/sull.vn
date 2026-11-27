use wasm_bindgen::prelude::*;

/// What a cell on the board holds.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    /// Nothing.
    Empty = 0,

    /// Food for any snake.
    Food = 1,

    /// Part of your snake, behind its head.
    You = 2,

    /// Your snake's head.
    YourHead = 3,

    /// Part of a clone, behind its head.
    Clone = 4,

    /// A clone's head.
    CloneHead = 5,
}

impl From<Cell> for u8 {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Empty => 0,
            Cell::Food => 1,
            Cell::You => 2,
            Cell::YourHead => 3,
            Cell::Clone => 4,
            Cell::CloneHead => 5,
        }
    }
}
