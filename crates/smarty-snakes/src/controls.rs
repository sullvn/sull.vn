use wasm_bindgen::prelude::*;

use crate::turn::Turn;
use crate::world::Direction;

///
/// What you pressed or tapped since the last move.
///
/// Arrows point on the screen. Taps turn relative to
/// your snake's heading.
///
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    /// Nothing, so your snake goes straight.
    None = 0,

    /// The up arrow.
    Up = 1,

    /// The right arrow.
    Right = 2,

    /// The down arrow.
    Down = 3,

    /// The left arrow.
    Left = 4,

    /// A tap on the left half of the board.
    TapLeft = 5,

    /// A tap on the right half of the board.
    TapRight = 6,
}

///
/// The move a moving snake makes for `input`.
///
/// The arrow behind the snake is ignored, as it can't
/// reverse.
///
pub(crate) fn steer(input: Input, heading: Direction) -> Turn {
    match input {
        Input::TapLeft => Turn::Left,
        Input::TapRight => Turn::Right,
        _ => arrow_turn(input, heading).unwrap_or(Turn::Straight),
    }
}

///
/// The move a parked snake wakes with, if `input` wakes
/// it.
///
/// A tap only wakes it, so it sets off straight. The
/// arrow behind it doesn't wake it at all.
///
pub(crate) fn wake(input: Input, heading: Direction) -> Option<Turn> {
    match input {
        Input::TapLeft | Input::TapRight => Some(Turn::Straight),
        _ => arrow_turn(input, heading),
    }
}

/// The turn toward an arrow, unless it's behind the snake.
fn arrow_turn(input: Input, heading: Direction) -> Option<Turn> {
    let direction = match input {
        Input::Up => Direction::North,
        Input::Right => Direction::East,
        Input::Down => Direction::South,
        Input::Left => Direction::West,
        Input::None | Input::TapLeft | Input::TapRight => return None,
    };

    heading.turn_to(direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: [Input; 7] = [
        Input::None,
        Input::Up,
        Input::Right,
        Input::Down,
        Input::Left,
        Input::TapLeft,
        Input::TapRight,
    ];

    #[test]
    fn steer() {
        assert_eq!(
            INPUTS.map(|input| super::steer(input, Direction::North)),
            [
                Turn::Straight,
                Turn::Straight,
                Turn::Right,
                Turn::Straight,
                Turn::Left,
                Turn::Left,
                Turn::Right,
            ],
            "steer goes straight for nothing, the forward arrow, or the reverse arrow"
        );
    }

    #[test]
    fn steer_relative() {
        assert_eq!(
            [Input::Up, Input::Down, Input::TapLeft]
                .map(|input| super::steer(input, Direction::East)),
            [Turn::Left, Turn::Right, Turn::Left],
            "steer turns arrows by the heading, and taps by the snake's own left and right"
        );
    }

    #[test]
    fn wake() {
        assert_eq!(
            INPUTS.map(|input| super::wake(input, Direction::North)),
            [
                None,
                Some(Turn::Straight),
                Some(Turn::Right),
                None,
                Some(Turn::Left),
                Some(Turn::Straight),
                Some(Turn::Straight),
            ],
            "wake ignores nothing and the reverse arrow, and taps only wake"
        );
    }
}
