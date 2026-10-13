//! A small crate that exercises the workspace's toolchain, lints, and tests.

use std::num::ParseIntError;

/// Returns a greeting for `name`.
///
/// ```
/// assert_eq!(smoke::greet("Ada"), "Hello, Ada!", "greets by name");
/// ```
///
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Adds up `values`.
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
pub fn parse_byte(s: &str) -> Result<u8, ParseIntError> {
    s.parse()
}

/// Counts up from zero.
#[derive(Debug, Default)]
pub struct Counter {
    count: u32,
}

impl Counter {
    /// Creates a counter at zero.
    #[must_use]
    pub const fn new() -> Self {
        Self { count: 0 }
    }

    /// Adds one, saturating at `u32::MAX`.
    pub const fn increment(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    /// Returns the current count.
    #[must_use]
    pub const fn value(&self) -> u32 {
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
        assert!(parse_byte("256").is_err(), "rejects numbers over 255");
    }

    #[test]
    fn counts() {
        let mut counter = Counter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.value(), 2, "counts each increment");
    }
}
