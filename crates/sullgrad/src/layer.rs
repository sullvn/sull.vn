use super::error::InputLengthError;
use super::expression::Expression;
use super::neuron::Neuron;

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
    fn forward() {
        let layer = Layer::from_fn(2, 2, index_as_value);
        let inputs = [1.0, -1.0].map(Expression::from);
        let data: Result<Vec<_>, _> = layer
            .forward(&inputs)
            .map(|outputs| outputs.iter().map(Expression::data).collect());

        assert_eq!(
            data,
            Ok(vec![1.0_f64.tanh(), 4.0_f64.tanh()]),
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
        let error = layer.forward(&inputs).err();

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
