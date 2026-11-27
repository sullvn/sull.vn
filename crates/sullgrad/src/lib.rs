//! A humble micrograd implementation
pub mod error;
mod expression;
mod layer;
mod loss;
mod math;
mod mlp;
mod model;
mod neuron;

#[cfg(test)]
mod testing;

pub use model::{Model, Sample};
