use super::error::LengthError;
use super::expression::Expression;

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Neuron {
    weights: Box<[Expression]>,
    bias: Expression,
}

impl Neuron {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_fn<P: Into<Expression>>(
        nin: usize,
        mut parameter: impl FnMut(usize) -> P,
    ) -> Self {
        let weights = (0..nin).map(&mut parameter).map(Into::into).collect();
        let bias = parameter(nin).into();

        Self { weights, bias }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_parameters(parameters: &[f64]) -> Result<Self, LengthError> {
        let (&bias, weights) = parameters.split_last().ok_or(LengthError::Parameters)?;
        let weights = weights.iter().copied().map(Expression::from).collect();
        let bias = bias.into();

        Ok(Self { weights, bias })
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn forward(&self, inputs: &[Expression]) -> Result<Expression, LengthError> {
        if inputs.len() != self.weights.len() {
            return Err(LengthError::Inputs);
        }

        let weighted_sum: Expression = self
            .weights
            .iter()
            .zip(inputs)
            .map(|(w, x)| w.clone() * x.clone())
            .sum();

        let pre_activation = weighted_sum + self.bias.clone();
        let activation = pre_activation.tanh();

        Ok(activation)
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn parameters(&self) -> impl Iterator<Item = &Expression> {
        self.weights.iter().chain([&self.bias])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::index_as_value;

    #[test]
    fn from_parameters() {
        let parameters = [0.0, 1.0, 2.0];
        let rebuilt: Vec<_> = Neuron::from_parameters(&parameters)
            .unwrap()
            .parameters()
            .map(Expression::data)
            .collect();

        assert_eq!(
            rebuilt, parameters,
            "from_parameters round-trips through parameters"
        );
    }

    #[test]
    fn forward() {
        let neuron = Neuron::from_parameters(&[1.0, 2.0, 3.0]).unwrap();
        let inputs = [1.0, -1.0].map(Expression::from);
        let output = neuron.forward(&inputs).unwrap();

        assert_eq!(
            output.data(),
            2.0_f64.tanh(),
            "forward computes tanh(w · x + b)"
        );
    }

    #[test]
    fn parameters() {
        let neuron = Neuron::from_fn(2, index_as_value);
        let data: Vec<_> = neuron.parameters().map(Expression::data).collect();

        assert_eq!(
            data,
            [0.0, 1.0, 2.0],
            "weights and bias are returned as parameters"
        );
    }

    #[test]
    fn input_length_mismatch() {
        let neuron = Neuron::from_fn(2, |_| 0.0);
        let inputs = [3.0].map(Expression::from);
        let error = neuron.forward(&inputs).unwrap_err();

        assert_eq!(
            error,
            LengthError::Inputs,
            "forward rejects inputs of the wrong length"
        );
    }

    #[test]
    fn parameters_length_mismatch() {
        let error = Neuron::from_parameters(&[]).unwrap_err();

        assert_eq!(
            error,
            LengthError::Parameters,
            "from_parameters rejects empty parameters"
        );
    }
}
