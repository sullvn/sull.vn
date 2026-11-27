///
/// Picks one of `items` with `random`, which returns a
/// number from 0 to 1.
///
/// Returns `None` only if `items` is empty.
///
pub(crate) fn pick<'a, T>(random: &mut impl FnMut() -> f64, items: &'a [T]) -> Option<&'a T> {
    let count = f64::from(u32::try_from(items.len()).ok()?);
    let position = (random() * count).floor();

    // Rust has no fallible float-to-integer conversion. `as`
    // saturates instead, and `get` catches anything past the end.
    #[expect(
        clippy::as_conversions,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "saturating is safe here"
    )]
    let index = position as usize;

    items.get(index).or_else(|| items.last())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::scripted;

    #[test]
    fn picks_by_position() {
        let mut random = scripted(&[0.0, 0.5, 0.99]);
        let items = ['a', 'b', 'c'];
        let picks: Vec<_> = (0..3).map(|_| pick(&mut random, &items)).collect();

        assert_eq!(
            picks,
            [Some(&'a'), Some(&'b'), Some(&'c')],
            "pick maps 0 to 1 evenly across items"
        );
    }

    #[test]
    fn one_picks_last() {
        let mut random = scripted(&[1.0]);

        assert_eq!(
            pick(&mut random, &['a', 'b']),
            Some(&'b'),
            "pick treats 1 as the last item"
        );
    }

    #[test]
    fn empty() {
        let mut random = scripted(&[0.5]);
        let no_items: [char; 0] = [];

        assert_eq!(
            pick(&mut random, &no_items),
            None,
            "pick has nothing to pick from no items"
        );
    }
}
