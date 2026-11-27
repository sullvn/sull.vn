//! A snake game where every other snake learns from you
mod cell;
mod random;
mod tuning;
mod turn;
mod world;

#[cfg(test)]
mod testing;

pub use cell::Cell;
