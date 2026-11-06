use super::error::LengthError;
use super::expression::Expression;
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

    fn mlp(&self) -> Result<Mlp, LengthError> {
        Mlp::from_parameters(self.nin, &self.nouts, &self.parameters)
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn parameters(&self) -> &[f64] {
        &self.parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::index_as_value;

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
}
