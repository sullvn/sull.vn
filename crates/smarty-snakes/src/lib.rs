//! A snake game where every other snake learns from the player
mod bindings;
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

pub use bindings::Game;
pub use cell::Cell;
pub use controls::Input;
