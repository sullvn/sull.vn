use wasm_bindgen::prelude::*;

/// What a cell on the board holds
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    /// Nothing
    Empty = 0,

    /// Food for any snake
    Food = 1,

    /// Part of the player's tail
    PlayerTail = 2,

    /// The player's head
    PlayerHead = 3,

    /// Part of a bot's tail
    BotTail = 4,

    /// A bot's head
    BotHead = 5,
}

impl From<Cell> for u8 {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Empty => 0,
            Cell::Food => 1,
            Cell::PlayerTail => 2,
            Cell::PlayerHead => 3,
            Cell::BotTail => 4,
            Cell::BotHead => 5,
        }
    }
}
