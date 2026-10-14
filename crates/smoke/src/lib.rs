//! A small crate that exercises the workspace's toolchain, lints, tests, and wasm-bindgen exports.

use wasm_bindgen::prelude::*;

/// Returns a greeting for `name`.
///
/// ```
/// assert_eq!(smoke::greet("Ada"), "Hello, Ada!", "greets by name");
/// ```
///
#[wasm_bindgen]
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Adds up `values`.
#[wasm_bindgen]
#[must_use]
pub fn sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

/// Parses a decimal byte.
///
/// # Errors
///
/// If `s` is not a number from 0 to 255.
///
#[wasm_bindgen]
pub fn parse_byte(s: &str) -> Result<u8, JsError> {
    Ok(s.parse()?)
}

/// Counts up from zero.
#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Counter {
    count: u32,
}

#[wasm_bindgen]
#[expect(clippy::missing_const_for_fn, reason = "wasm-bindgen rejects const fn")]
impl Counter {
    /// Creates a counter at zero.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one, saturating at `u32::MAX`.
    pub fn increment(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    /// The current count.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn value(&self) -> u32 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(greet("Ada"), "Hello, Ada!", "greets by name");
    }

    #[test]
    fn sums() {
        assert!((sum(&[1.5, 2.5]) - 4.0).abs() < f64::EPSILON, "adds values");
        assert!(sum(&[]) == 0.0, "empty sum is zero");
    }

    #[test]
    fn parses_bytes() {
        assert_eq!(parse_byte("255").unwrap(), 255, "parses the largest byte");
    }

    #[test]
    fn counts() {
        let mut counter = Counter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 2, "counts each increment");
    }
}
