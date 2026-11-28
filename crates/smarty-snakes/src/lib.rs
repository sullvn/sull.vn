//!
//! A snake game where every other snake learns from the player
//!
//! - `world` holds the board and moves the snakes. It
//!   knows nothing about learning.
//!
//! - `senses` turns what a snake can see into numbers.
//!
//! - `brain` is one network that every bot shares. It
//!   trains on what the player sensed and how they
//!   turned.
//!
//! - `game` plays a move: it steers the player, asks the
//!   brain for each bot's turn, moves the world, then
//!   trains the brain.
//!
//! - `bindings` exports the game to JavaScript.
//!

mod bindings;
mod brain;
mod cell;
mod controls;
mod game;
mod senses;
mod tuning;
mod turn;
mod world;

#[cfg(test)]
mod testing;

pub use bindings::Game;
pub use cell::Cell;
pub use controls::Input;
