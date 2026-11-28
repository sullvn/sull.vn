//! A snake game where every other snake learns from you
use wasm_bindgen::prelude::*;

mod brain;
mod cell;
mod controls;
mod game;
mod random;
mod senses;
mod tuning;
mod turn;
mod world;

#[cfg(test)]
mod testing;

pub use cell::Cell;
pub use controls::Input;

///
/// A game of Smarty Snakes.
///
/// Draws its chance from `Math.random`.
///
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game(game::Game<fn() -> f64>);

#[wasm_bindgen]
impl Game {
    ///
    /// A new game on a board `width` by `height` cells,
    /// with you parked.
    ///
    /// # Errors
    ///
    /// If the board is too small for snakes to spawn.
    ///
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Result<Self, JsError> {
        let board = world::Board::new(width, height)
            .ok_or_else(|| JsError::new("The board is too small for snakes to spawn"))?;
        let random: fn() -> f64 = js_sys::Math::random;

        Ok(Self(game::Game::new(board, random)))
    }

    ///
    /// Plays one move, with `input` steering you.
    ///
    /// Does nothing if you're parked and `input` doesn't
    /// wake you.
    ///
    /// # Errors
    ///
    /// If the brain's shape doesn't fit what snakes sense,
    /// which is a bug.
    ///
    pub fn tick(&mut self, input: Input) -> Result<(), JsError> {
        let Self(game) = self;

        game.tick(input)
            .map_err(|error| JsError::new(&format!("Brain error: {error:?}")))
    }

    /// Whether you're parked, so nothing moves until you do.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn parked(&self) -> bool {
        let Self(game) = self;
        game.parked()
    }

    /// The board as one [`Cell`] code per cell, row by row.
    #[must_use]
    pub fn cells(&self) -> Box<[u8]> {
        let Self(game) = self;
        game.cells()
    }
}
