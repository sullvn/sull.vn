use super::error::InputLengthError;
use super::expression::Expression;

#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Neuron {
    weights: Box<[Expression]>,
    bias: Expression,
}

impl Neuron {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_fn(nin: usize, mut parameter: impl FnMut(usize) -> f64) -> Self {
        let weights = (0..nin).map(&mut parameter).map(Expression::from).collect();
        let bias = parameter(nin).into();

        Self { weights, bias }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn forward(&self, inputs: &[Expression]) -> Result<Expression, InputLengthError> {
        if inputs.len() != self.weights.len() {
            return Err(InputLengthError {
                expected: self.weights.len(),
                actual: inputs.len(),
            });
        }

        let activation = self
            .weights
            .iter()
            .zip(inputs)
            .fold(self.bias.clone(), |sum, (w, x)| sum + w.clone() * x.clone());

        Ok(activation.tanh())
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
    fn forward() {
        let neuron = Neuron::from_fn(2, index_as_value);
        let inputs = [3.0, 4.0].map(Expression::from);
        let output = neuron.forward(&inputs).map(|o| o.data());

        assert_eq!(
            output,
            Ok(6.0_f64.tanh()),
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
        let error = neuron.forward(&inputs).err();

        assert_eq!(
            error,
            Some(InputLengthError {
                expected: 2,
                actual: 1
            }),
            "forward rejects inputs of the wrong length"
        );
    }
}
