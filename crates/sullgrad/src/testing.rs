use std::array;

use crate::expression::Expression;

pub(crate) fn index_as_value(i: usize) -> f64 {
    u32::try_from(i).map_or(f64::NAN, f64::from)
}

///
/// Gradients of `f` at `at`, computed by
/// backpropagation.
///
pub(crate) fn backward_gradients<const N: usize>(
    f: impl Fn([Expression; N]) -> Expression,
    at: [f64; N],
) -> [f64; N] {
    let inputs = at.map(Expression::from);
    f(inputs.clone()).backward();

    inputs.map(|input| input.grad())
}

///
/// Gradients of `f` at `at`, estimated from its slope.
///
/// Nudges each input a little each way and measures
/// how much the output moves. Only needs the forward
/// pass, so it's an independent check on `backward`.
///
pub(crate) fn numerical_gradients<const N: usize>(
    f: impl Fn([Expression; N]) -> Expression,
    at: [f64; N],
) -> [f64; N] {
    // Small enough for a good estimate, but not so
    // small that rounding error takes over
    const H: f64 = 1e-6;

    array::from_fn(|i| {
        let nudged = |h: f64| {
            let mut at = at;
            at[i] += h;
            f(at.map(Expression::from)).data()
        };

        (nudged(H) - nudged(-H)) / (2.0 * H)
    })
}

///
/// Asserts each pair of values is nearly equal.
///
/// Relative to the values' size, so large and small
/// gradients get the same treatment, but absolute
/// for values under `1`, where relative error blows
/// up.
///
#[track_caller]
pub(crate) fn assert_approx_eq<const N: usize>(
    actual: [f64; N],
    expected: [f64; N],
    message: &str,
) {
    const TOLERANCE: f64 = 1e-6;

    let close = actual
        .iter()
        .zip(&expected)
        .all(|(a, e)| (a - e).abs() <= TOLERANCE * a.abs().max(e.abs()).max(1.0));

    assert!(close, "{message}: {actual:?} is not close to {expected:?}");
}
