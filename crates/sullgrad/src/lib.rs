//! A humble micrograd implementation
mod error;
mod expression;
mod layer;
mod loss;
mod math;
mod mlp;
mod model;
mod neuron;

#[cfg(test)]
mod testing;

pub use error::LengthError;
pub use model::{Model, Sample};
