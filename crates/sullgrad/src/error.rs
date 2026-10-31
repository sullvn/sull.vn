#[derive(Debug, PartialEq, Eq)]
pub(crate) struct InputLengthError {
    pub(crate) expected: usize,
    pub(crate) actual: usize,
}
