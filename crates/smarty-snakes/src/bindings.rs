use rand::SeedableRng;
use rand::rngs::SmallRng;
use wasm_bindgen::prelude::*;

use crate::controls::Input;
use crate::game;
use crate::world::Size;

///
/// A game of Smarty Snakes.
///
/// Seeds its chance from `Math.random`.
///
#[wasm_bindgen]
#[derive(Debug)]
pub struct Game(game::Game<SmallRng>);

#[wasm_bindgen]
impl Game {
    ///
    /// A new game on a board `width` by `height` cells,
    /// with the player parked.
    ///
    /// # Errors
    ///
    /// If the board is too small for snakes to spawn.
    ///
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16) -> Result<Self, JsError> {
        let size = Size::new(width, height)
            .ok_or_else(|| JsError::new("The board is too small for snakes to spawn"))?;
        let rng = SmallRng::seed_from_u64(js_sys::Math::random().to_bits());

        Ok(Self(game::Game::new(size, rng)))
    }

    ///
    /// Plays one move, with `input` steering the player.
    ///
    /// Does nothing if the player is parked and `input`
    /// doesn't wake them.
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

    /// Whether the player is parked, so nothing moves until they do.
    #[wasm_bindgen(getter)]
    #[must_use]
    #[expect(clippy::missing_const_for_fn, reason = "wasm-bindgen rejects const fn")]
    pub fn parked(&self) -> bool {
        let Self(game) = self;
        game.parked()
    }

    /// The board as one [`Cell`](crate::Cell) code per cell, row by row.
    #[must_use]
    pub fn cells(&self) -> Box<[u8]> {
        let Self(game) = self;
        game.cells()
    }
}
