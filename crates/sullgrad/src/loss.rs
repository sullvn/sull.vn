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
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) fn mean_squared_error(
    predictions: &[Expression],
    targets: &[f64],
) -> Result<Expression, LengthError> {
    if predictions.is_empty() || predictions.len() != targets.len() {
        return Err(LengthError::Targets);
    }

    let count = u32::try_from(targets.len()).map_err(|_| LengthError::Targets)?;

    let sum: Expression = predictions
        .iter()
        .zip(targets)
        .map(|(prediction, &target)| {
            let error = prediction.clone() - target.into();

            // Use multiplication instead of `pow(2)`, as
            // that would give `NaN` gradients for negative
            // errors
            error.clone() * error
        })
        .sum();

    Ok(sum / f64::from(count).into())
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
