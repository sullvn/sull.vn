use super::error::LengthError;
use super::expression::Expression;

///
/// Mean squared error.
///
/// Averages the squared difference between each
/// prediction and its target. Squaring makes every
/// error positive and penalizes large misses the
/// most.
///
/// Named after Keras's `mean_squared_error`.
///
/// # Gradient
///
/// ```text
/// L       = ((p_1 - t_1)^2 + ... + (p_n - t_n)^2) / n
/// ∂L/∂p_i = 2 * (p_i - t_i) / n
/// ```
///
/// Each prediction's gradient points away from its
/// target, scaled by how far it misses.
///
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) fn mean_squared_error(
    predictions: &[Expression],
    targets: &[f64],
) -> Result<Expression, LengthError> {
    if predictions.is_empty() || predictions.len() != targets.len() {
        return Err(LengthError::Targets);
    }

    let count_integer = u32::try_from(targets.len()).map_err(|_| LengthError::Targets)?;
    let count = f64::from(count_integer);

    let sum: Expression = predictions
        .iter()
        .zip(targets)
        .map(|(prediction, &target)| {
            let error = prediction - target;

            // Use multiplication instead of `pow(2)`, as
            // that would give `NaN` gradients for negative
            // errors
            &error * &error
        })
        .sum();

    let mse = sum / count;
    Ok(mse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_squared_error() {
        let predictions = [1.0, 2.0].map(Expression::from);
        let loss = super::mean_squared_error(&predictions, &[0.0, 4.0]).unwrap();

        assert_eq!(loss.data(), 2.5, "mean of squared errors 1² and (-2)²");
    }

    #[test]
    fn gradient() {
        let predictions = [1.0, 2.0].map(Expression::from);
        let targets = [0.0, 4.0];

        super::mean_squared_error(&predictions, &targets)
            .unwrap()
            .backward();
        let grads: Vec<_> = predictions.iter().map(Expression::grad).collect();

        assert_eq!(
            grads,
            [1.0, -2.0],
            "each prediction's gradient is 2 * error / count"
        );
    }

    #[test]
    fn targets_length_mismatch() {
        let predictions = [1.0, 2.0].map(Expression::from);
        let error = super::mean_squared_error(&predictions, &[0.0]).unwrap_err();

        assert_eq!(
            error,
            LengthError::Targets,
            "mean_squared_error rejects a target count that differs from predictions"
        );
    }

    #[test]
    fn empty() {
        let error = super::mean_squared_error(&[], &[]).unwrap_err();

        assert_eq!(
            error,
            LengthError::Targets,
            "mean_squared_error rejects empty predictions, whose mean is NaN"
        );
    }
}
