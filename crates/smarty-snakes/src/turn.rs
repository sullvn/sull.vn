///
/// A move, relative to a snake's heading.
///
/// Also the brain's three choices, in the order of its
/// outputs.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    pub(crate) const ALL: [Self; 3] = [Self::Left, Self::Straight, Self::Right];

    ///
    /// What the brain should output for this move.
    ///
    /// High for the move taken, and low for the others.
    ///
    pub(crate) const fn targets(self) -> &'static [f64; 3] {
        match self {
            Self::Left => &[1.0, -1.0, -1.0],
            Self::Straight => &[-1.0, 1.0, -1.0],
            Self::Right => &[-1.0, -1.0, 1.0],
        }
    }
}
