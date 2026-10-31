pub(crate) fn index_as_value(i: usize) -> f64 {
    u32::try_from(i).map_or(f64::NAN, f64::from)
}
