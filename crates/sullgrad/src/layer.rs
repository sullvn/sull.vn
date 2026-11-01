use super::error::{InputLengthError, ParametersLengthError};
use super::expression::Expression;
use super::neuron::Neuron;

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Layer {
    neurons: Box<[Neuron]>,
}

impl Layer {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_fn(
        nin: usize,
        nout: usize,
        mut parameter: impl FnMut(usize) -> f64,
    ) -> Self {
        let mut offset = 0;
        let neurons = (0..nout)
            .map(|_| {
                let neuron = Neuron::from_fn(nin, |i| parameter(offset + i));
                offset += neuron.parameters().count();
                neuron
            })
            .collect();

        Self { neurons }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_parameters(
        nin: usize,
        parameters: &[f64],
    ) -> Result<Self, ParametersLengthError> {
        let chunks = parameters.chunks_exact(nin + 1);
        if !chunks.remainder().is_empty() {
            return Err(ParametersLengthError);
        }

        let neurons = chunks
            .map(Neuron::from_parameters)
            .collect::<Result<_, _>>()?;

        Ok(Self { neurons })
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn forward(
        &self,
        inputs: &[Expression],
    ) -> Result<Box<[Expression]>, InputLengthError> {
        self.neurons
            .iter()
            .map(|neuron| neuron.forward(inputs))
            .collect()
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn parameters(&self) -> impl Iterator<Item = &Expression> {
        self.neurons.iter().flat_map(Neuron::parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::index_as_value;

    #[test]
    fn from_parameters() {
        let parameters = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let rebuilt: Vec<_> = Layer::from_parameters(2, &parameters)
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
        let layer = Layer::from_parameters(2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let inputs = [1.0, -1.0].map(Expression::from);
        let outputs = layer.forward(&inputs).unwrap();
        let data: Vec<_> = outputs.iter().map(Expression::data).collect();

        assert_eq!(
            data,
            [2.0_f64.tanh(), 5.0_f64.tanh()],
            "forward computes each neuron's output"
        );
    }

    #[test]
    fn parameters() {
        let layer = Layer::from_fn(2, 2, index_as_value);
        let data: Vec<_> = layer.parameters().map(Expression::data).collect();

        assert_eq!(
            data,
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            "each neuron's parameters are returned in order"
        );
    }

    #[test]
    fn input_length_mismatch() {
        let layer = Layer::from_fn(2, 2, |_| 0.0);
        let inputs = [3.0].map(Expression::from);
        let error = layer.forward(&inputs).unwrap_err();

        assert_eq!(
            error,
            InputLengthError {
                expected: 2,
                actual: 1
            },
            "forward rejects inputs of the wrong length"
        );
    }

    #[test]
    fn parameters_length_mismatch() {
        let error = Layer::from_parameters(2, &[0.0; 5]).unwrap_err();

        assert_eq!(
            error, ParametersLengthError,
            "from_parameters rejects a partial neuron"
        );
    }
}
