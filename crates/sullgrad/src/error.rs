///
/// A slice had the wrong length for the model.
///
#[derive(Debug, PartialEq, Eq)]
pub enum LengthError {
    /// The batch was empty, or too large to average.
    Batch,

    /// The inputs didn't match the model's input count.
    Inputs,

    /// The parameters didn't match the model's shape.
    Parameters,

    /// The targets didn't match the model's output count.
    Targets,
}
