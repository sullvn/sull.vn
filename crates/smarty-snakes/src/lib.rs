//! A snake game where every other snake learns from you
mod brain;
mod cell;
mod controls;
mod random;
mod senses;
mod tuning;
mod turn;
mod world;

#[cfg(test)]
mod testing;

pub use cell::Cell;
pub use controls::Input;
