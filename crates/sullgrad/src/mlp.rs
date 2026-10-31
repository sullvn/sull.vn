use super::error::InputLengthError;
use super::expression::Expression;
use super::layer::Layer;

#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Mlp {
    layers: Box<[Layer]>,
}

impl Mlp {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_fn(
        nin: usize,
        nouts: &[usize],
        mut parameter: impl FnMut(usize) -> f64,
    ) -> Self {
        let sizes = [&[nin], nouts].concat();
        let mut offset = 0;
        let layers = sizes
            .array_windows()
            .map(|&[nin, nout]| {
                let layer = Layer::from_fn(nin, nout, |i| parameter(offset + i));
                offset += layer.parameters().count();
                layer
            })
            .collect();

        Self { layers }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn forward(
        &self,
        inputs: &[Expression],
    ) -> Result<Box<[Expression]>, InputLengthError> {
        let mut outputs: Box<[_]> = inputs.into();
        for layer in &self.layers {
            outputs = layer.forward(&outputs)?;
        }

        Ok(outputs)
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn parameters(&self) -> impl Iterator<Item = &Expression> {
        self.layers.iter().flat_map(Layer::parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::index_as_value;

    #[test]
    #[expect(
        clippy::suboptimal_flops,
        reason = "mul_add would round differently than the separate multiply and add in forward"
    )]
    fn forward() {
        let mlp = Mlp::from_fn(2, &[2, 1], index_as_value);
        let inputs = [1.0, -1.0].map(Expression::from);
        let data: Result<Vec<_>, _> = mlp
            .forward(&inputs)
            .map(|outputs| outputs.iter().map(Expression::data).collect());

        let [h0, h1] = [1.0_f64.tanh(), 4.0_f64.tanh()];
        let output = (8.0 + 6.0 * h0 + 7.0 * h1).tanh();

        assert_eq!(
            data,
            Ok(vec![output]),
            "forward feeds each layer's outputs into the next"
        );
    }

    #[test]
    fn parameters() {
        let mlp = Mlp::from_fn(2, &[2, 1], index_as_value);
        let data: Vec<_> = mlp.parameters().map(Expression::data).collect();

        assert_eq!(
            data,
            [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            "each layer's parameters are returned in order"
        );
    }

    #[test]
    fn input_length_mismatch() {
        let mlp = Mlp::from_fn(2, &[2, 1], |_| 0.0);
        let inputs = [3.0].map(Expression::from);
        let error = mlp.forward(&inputs).err();

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
