use super::error::LengthError;
use super::expression::Expression;
use super::layer::Layer;

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
pub(crate) struct Mlp {
    layers: Box<[Layer]>,
}

impl Mlp {
    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn from_fn<P: Into<Expression>>(
        nin: usize,
        nouts: &[usize],
        mut parameter: impl FnMut(usize) -> P,
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
    pub(crate) fn from_parameters(
        nin: usize,
        nouts: &[usize],
        parameters: &[f64],
    ) -> Result<Self, LengthError> {
        let mlp = Self::from_fn(nin, nouts, |i| {
            parameters.get(i).copied().unwrap_or(f64::NAN)
        });
        if mlp.parameters().count() != parameters.len() {
            return Err(LengthError::Parameters);
        }

        Ok(mlp)
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "wip"))]
    pub(crate) fn forward(&self, inputs: &[Expression]) -> Result<Box<[Expression]>, LengthError> {
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
    fn from_parameters() {
        let parameters = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let rebuilt: Vec<_> = Mlp::from_parameters(2, &[2, 1], &parameters)
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
    #[expect(
        clippy::suboptimal_flops,
        reason = "mul_add would round differently than the separate multiply and add in forward"
    )]
    fn forward() {
        let parameters = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mlp = Mlp::from_parameters(2, &[2, 1], &parameters).unwrap();
        let inputs = [1.0, -1.0].map(Expression::from);
        let outputs = mlp.forward(&inputs).unwrap();
        let data: Vec<_> = outputs.iter().map(Expression::data).collect();

        let [h0, h1] = [2.0_f64.tanh(), 5.0_f64.tanh()];
        let output = (9.0 + 7.0 * h0 + 8.0 * h1).tanh();

        assert_eq!(
            data,
            [output],
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
        let error = mlp.forward(&inputs).unwrap_err();

        assert_eq!(
            error,
            LengthError::Inputs,
            "forward rejects inputs of the wrong length"
        );
    }

    #[test]
    fn parameters_length_mismatch() {
        let error = Mlp::from_parameters(2, &[2, 1], &[0.0; 8]).unwrap_err();

        assert_eq!(
            error,
            LengthError::Parameters,
            "from_parameters rejects the wrong number of parameters"
        );
    }
}
