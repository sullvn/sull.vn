use super::error::LengthError;
use super::expression::Expression;
use super::loss::mean_squared_error;
use super::math;
use super::mlp::Mlp;

///
/// A deep learning model.
///
/// Includes stored weights and biases.
///
/// The neural network structure is rebuilt on
/// demand, initializing itself from the stored
/// weights and biases.
///
/// Currently only supports an [`Mlp`] structure.
///
#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Model {
    nin: usize,
    nouts: Box<[usize]>,
    parameters: Box<[f64]>,
}

impl Model {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_fn(
        nin: usize,
        nouts: &[usize],
        parameter: impl FnMut(usize) -> f64,
    ) -> Self {
        let parameters = Mlp::from_fn(nin, nouts, parameter)
            .parameters()
            .map(Expression::data)
            .collect();

        Self {
            nin,
            nouts: nouts.into(),
            parameters,
        }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_parameters(
        nin: usize,
        nouts: &[usize],
        parameters: &[f64],
    ) -> Result<Self, LengthError> {
        //
        // Built for validation.
        //
        // TODO: This is relatively expensive. Revisit after all
        // of this moves to a new architecture, such as SoA.
        //
        Mlp::from_parameters(nin, nouts, parameters)?;

        Ok(Self {
            nin,
            nouts: nouts.into(),
            parameters: parameters.into(),
        })
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn forward(&self, inputs: &[f64]) -> Result<Box<[f64]>, LengthError> {
        let inputs: Box<[_]> = inputs.iter().copied().map(Expression::from).collect();
        let outputs = self.mlp()?.forward(&inputs)?;

        Ok(outputs.iter().map(Expression::data).collect())
    }

    ///
    /// One step of gradient descent.
    ///
    /// Nudges each parameter against its gradient,
    /// lowering the loss on `batch`. Returns the loss
    /// from before the nudge.
    ///
    /// Named after Keras's `Model.train_step`.
    ///
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn train_step(
        &mut self,
        batch: &[Sample<'_>],
        learning_rate: f64,
    ) -> Result<f64, LengthError> {
        let mlp = self.mlp()?;
        let loss = batch_loss(&mlp, batch)?;
        loss.backward();

        for (value, parameter) in self.parameters.iter_mut().zip(mlp.parameters()) {
            *value = math::mul_add(-learning_rate, parameter.grad(), *value);
        }

        Ok(loss.data())
    }

    fn mlp(&self) -> Result<Mlp, LengthError> {
        Mlp::from_parameters(self.nin, &self.nouts, &self.parameters)
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn parameters(&self) -> &[f64] {
        &self.parameters
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Sample<'a> {
    pub(crate) inputs: &'a [f64],
    pub(crate) targets: &'a [f64],
}

///
/// Mean of each sample's mean squared error.
///
/// Averaged rather than summed, so the learning rate
/// means the same thing for any batch size.
///
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
fn batch_loss(mlp: &Mlp, batch: &[Sample<'_>]) -> Result<Expression, LengthError> {
    //
    // Error for empty batches.
    //
    // An empty batch averages to `NaN`, which
    // would spread and persist in downstream
    // calculations.
    //
    if batch.is_empty() {
        return Err(LengthError::Batch);
    }

    let count_integer = u32::try_from(batch.len()).map_err(|_| LengthError::Batch)?;
    let count = f64::from(count_integer);

    let sum = batch
        .iter()
        .map(|sample| {
            let inputs: Box<[_]> = sample
                .inputs
                .iter()
                .copied()
                .map(Expression::from)
                .collect();

            mean_squared_error(&mlp.forward(&inputs)?, sample.targets)
        })
        .sum::<Result<Expression, _>>()?;

    let loss = sum / count;
    Ok(loss)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{
        assert_approx_eq, backward_gradients, index_as_value, numerical_gradients,
    };

    // micrograd's toy dataset
    const TOY_BATCH: [Sample<'static>; 4] = [
        Sample {
            inputs: &[2.0, 3.0, -1.0],
            targets: &[1.0],
        },
        Sample {
            inputs: &[3.0, -1.0, 0.5],
            targets: &[-1.0],
        },
        Sample {
            inputs: &[0.5, 1.0, 1.0],
            targets: &[-1.0],
        },
        Sample {
            inputs: &[1.0, 1.0, -1.0],
            targets: &[1.0],
        },
    ];

    #[test]
    fn from_fn() {
        let model = Model::from_fn(2, &[2, 1], index_as_value);

        assert_eq!(
            model.parameters(),
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            "from_fn takes parameters in the same order as Mlp"
        );
    }

    #[test]
    fn from_parameters() {
        let parameters = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let model = Model::from_parameters(2, &[2, 1], &parameters).unwrap();

        assert_eq!(
            model.parameters(),
            parameters,
            "from_parameters round-trips through parameters"
        );
    }

    #[test]
    fn forward() {
        let model = Model::from_parameters(2, &[1], &[1.0, 2.0, 3.0]).unwrap();
        let outputs = model.forward(&[1.0, -1.0]).unwrap();

        // tanh(1 * 1 + 2 * -1 + 3) = tanh(2)
        assert_eq!(
            *outputs,
            [2.0_f64.tanh()],
            "forward computes the network's outputs"
        );
    }

    #[test]
    fn train_step() {
        let mut model = Model::from_parameters(1, &[1], &[0.0, 0.0]).unwrap();
        let batch = [Sample {
            inputs: &[1.0],
            targets: &[1.0],
        }];
        let loss = model.train_step(&batch, 0.1).unwrap();

        assert_eq!(loss, 1.0, "train_step returns the loss before updating");
        assert_eq!(
            model.parameters(),
            [0.2, 0.2],
            "train_step moves each parameter against its gradient"
        );
    }

    #[test]
    fn learns() {
        let mut model = Model::from_fn(3, &[4, 4, 1], |i| index_as_value(i).sin());
        let first = model.train_step(&TOY_BATCH, 0.1).unwrap();
        let last = (0..19)
            .map(|_| model.train_step(&TOY_BATCH, 0.1).unwrap())
            .last()
            .unwrap();

        assert!(first > 1.0, "the untrained loss starts high");
        assert!(last < 0.05, "20 train steps bring the loss close to zero");
    }

    #[test]
    fn gradient() {
        let loss = |parameters: [Expression; 41]| {
            let mlp = Mlp::from_fn(3, &[4, 4, 1], |i| parameters[i].clone());
            batch_loss(&mlp, &TOY_BATCH).unwrap()
        };
        let at = std::array::from_fn(|i| index_as_value(i).sin());

        assert_approx_eq(
            backward_gradients(loss, at),
            numerical_gradients(loss, at),
            "every parameter's gradient matches its numerical slope",
        );
    }

    #[test]
    fn input_length_mismatch() {
        let model = Model::from_fn(2, &[2, 1], |_| 0.0);
        let error = model.forward(&[3.0]).unwrap_err();

        assert_eq!(
            error,
            LengthError::Inputs,
            "forward rejects inputs of the wrong length"
        );
    }

    #[test]
    fn parameters_length_mismatch() {
        let error = Model::from_parameters(2, &[2, 1], &[0.0; 8]).unwrap_err();

        assert_eq!(
            error,
            LengthError::Parameters,
            "from_parameters rejects the wrong number of parameters"
        );
    }

    #[test]
    fn empty_batch() {
        let mut model = Model::from_fn(1, &[1], |_| 0.0);
        let error = model.train_step(&[], 0.1).unwrap_err();

        assert_eq!(
            error,
            LengthError::Batch,
            "train_step rejects an empty batch, whose loss is NaN"
        );
    }

    #[test]
    fn sample_length_mismatch() {
        let mut model = Model::from_fn(1, &[1], |_| 0.0);
        let inputs = model
            .train_step(
                &[Sample {
                    inputs: &[],
                    targets: &[1.0],
                }],
                0.1,
            )
            .unwrap_err();
        let targets = model
            .train_step(
                &[Sample {
                    inputs: &[1.0],
                    targets: &[],
                }],
                0.1,
            )
            .unwrap_err();

        assert_eq!(
            [inputs, targets],
            [LengthError::Inputs, LengthError::Targets],
            "train_step rejects samples of the wrong length"
        );
        assert_eq!(
            model.parameters(),
            [0.0, 0.0],
            "train_step leaves parameters unchanged on error"
        );
    }
}
