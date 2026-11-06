#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LengthError {
    Batch,
    Inputs,
    Parameters,
    Targets,
}
