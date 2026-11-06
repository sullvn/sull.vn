///
/// Computes `a * b + c`, maybe using optimizations.
///
/// In native code, FMA instructions execute this
/// in a single step.
///
/// WebAssembly doesn't have FMA instructions for
/// scalars. Using the built-in `mul_add` triggers
/// software emulation, which is 17x slower. So
/// we don't do it for WASM.
///
#[cfg(not(target_family = "wasm"))]
pub(crate) const fn mul_add(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)
}

#[cfg(target_family = "wasm")]
pub(crate) const fn mul_add(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}
