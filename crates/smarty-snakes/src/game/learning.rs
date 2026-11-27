use super::*;
use crate::testing::{food_seeker, seeded};
use crate::world::Slot;

/// Moves played in each game, enough for clones to learn.
const MOVES: usize = 300;

#[test]
fn clones_learn() {
    let learned = clone_meals(true);
    let unlearned = clone_meals(false);

    assert!(
        learned > 3 * unlearned,
        "clones of a player who heads for food eat far more than clones that never learned, \
         but ate {learned} to {unlearned}"
    );
}

///
/// Food eaten by clones while a player heads for food.
///
/// Without `learning`, the brain forgets every lesson,
/// so it never trains.
///
fn clone_meals(learning: bool) -> usize {
    let mut game = Game::new(Board::new(40, 30).unwrap(), seeded(1));
    let mut player = seeded(2);
    let mut meals = 0;

    for _ in 0..MOVES {
        if !learning {
            game.brain.memory.clear();
        }
        let food = game.world.food.clone();
        game.tick(food_seeker(&game.world, &mut player)).unwrap();

        meals += game
            .world
            .slots
            .iter()
            .skip(1)
            .filter_map(Slot::alive)
            .filter(|clone| food.contains(&clone.head))
            .count();
    }

    meals
}
