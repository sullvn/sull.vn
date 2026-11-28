use sullgrad::error::LengthError;

use crate::brain::{Brain, Lesson};
use crate::controls::{Input, steer, wake};
use crate::senses::sense;
use crate::turn::Turn;
use crate::world::{Size, World, YOU};

///
/// Snakes on a shared board, where every clone learns
/// from you.
///
/// The world only moves while you do. You start parked,
/// and park again whenever you die.
///
/// `random` returns a number from 0 to 1, and is the
/// game's only source of chance.
///
#[derive(Debug)]
pub(crate) struct Game<R> {
    world: World,
    brain: Brain,
    parked: bool,
    random: R,
}

impl<R: FnMut() -> f64> Game<R> {
    /// A new game, with you parked.
    pub(crate) fn new(size: Size, mut random: R) -> Self {
        let world = World::new(size, &mut random);
        let brain = Brain::new(&mut random);

        Self {
            world,
            brain,
            parked: true,
            random,
        }
    }

    /// Whether you're parked, so nothing moves.
    pub(crate) const fn parked(&self) -> bool {
        self.parked
    }

    /// The board as one cell code per cell, row by row.
    pub(crate) fn cells(&self) -> Box<[u8]> {
        self.world.cells()
    }

    ///
    /// Plays one move, with `input` steering you.
    ///
    /// Does nothing if you're parked and `input` doesn't
    /// wake you.
    ///
    pub(crate) fn tick(&mut self, input: Input) -> Result<(), LengthError> {
        let Some(heading) = self.world.snake(YOU).map(|you| you.heading) else {
            self.world.respawn(YOU, &mut self.random);
            return Ok(());
        };
        let turn = if self.parked {
            wake(input, heading)
        } else {
            Some(steer(input, heading))
        };
        let Some(turn) = turn else {
            return Ok(());
        };
        self.parked = false;

        self.world.count_down(&mut self.random);
        if let Some(you) = self.world.snake(YOU) {
            let senses = sense(&self.world, you);
            self.brain.remember(Lesson { senses, turn });
        }

        let turns = self.turns(turn)?;
        self.world.step(&turns, &mut self.random);
        if self.world.snake(YOU).is_none() {
            self.world.respawn(YOU, &mut self.random);
            self.parked = true;
        }

        self.brain.train(&mut self.random)
    }

    /// Each living snake's move: `yours` for you, and the brain's for clones.
    fn turns(&self, yours: Turn) -> Result<Vec<Option<Turn>>, LengthError> {
        self.world
            .snakes
            .iter()
            .enumerate()
            .map(|(index, state)| {
                let Some(snake) = state.alive() else {
                    return Ok(None);
                };
                if index == YOU {
                    return Ok(Some(yours));
                }

                self.brain.choose(&sense(&self.world, snake)).map(Some)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{self, SnakeCells, food_seeker, scripted, seeded, snake};
    use crate::world::{Direction, Point, SnakeState};

    fn game(snakes: Vec<SnakeCells>) -> Game<impl FnMut() -> f64> {
        Game {
            world: testing::world(10, 10, snakes),
            brain: Brain::new(&mut scripted(&[0.5])),
            parked: true,
            random: scripted(&[0.0]),
        }
    }

    fn head(game: &Game<impl FnMut() -> f64>, index: usize) -> Option<Point> {
        game.world.snake(index).map(|snake| snake.head)
    }

    #[test]
    fn starts_parked() {
        let game = Game::new(Size::new(20, 20).unwrap(), scripted(&[0.3, 0.6]));

        assert!(game.parked(), "new games start parked");
    }

    #[test]
    fn parked() {
        let mut game = game(vec![
            snake(&[(5, 5), (5, 6)], Direction::North),
            snake(&[(2, 2), (2, 3)], Direction::North),
        ]);
        let before = game.cells();
        game.tick(Input::None).unwrap();
        game.tick(Input::Down).unwrap();

        assert_eq!(
            (game.parked(), game.cells(), game.brain.memory.len()),
            (true, before, 0),
            "tick does nothing while parked, without an input that wakes you"
        );
    }

    #[test]
    fn wakes() {
        let mut game = game(vec![snake(&[(5, 5), (5, 6)], Direction::North)]);
        game.tick(Input::Left).unwrap();

        assert_eq!(
            (game.parked(), head(&game, YOU)),
            (false, Some(Point { x: 4, y: 5 })),
            "tick wakes you with an arrow, and makes that move"
        );
    }

    #[test]
    fn tap_wakes() {
        let mut game = game(vec![snake(&[(5, 5), (5, 6)], Direction::North)]);
        game.tick(Input::TapLeft).unwrap();

        assert_eq!(
            head(&game, YOU),
            Some(Point { x: 5, y: 4 }),
            "tick wakes you with a tap, which sets off straight"
        );
    }

    #[test]
    fn clones_move() {
        let mut game = game(vec![
            snake(&[(5, 5), (5, 6)], Direction::North),
            snake(&[(2, 2), (2, 3)], Direction::North),
        ]);
        game.tick(Input::Up).unwrap();

        assert_ne!(
            head(&game, 1),
            Some(Point { x: 2, y: 2 }),
            "tick moves clones as you move"
        );
    }

    #[test]
    fn lessons() {
        let mut game = game(vec![snake(&[(5, 5), (5, 6)], Direction::North)]);
        game.tick(Input::Up).unwrap();
        game.tick(Input::None).unwrap();
        game.tick(Input::Right).unwrap();
        let turns: Vec<_> = game.brain.memory.iter().map(|lesson| lesson.turn).collect();

        assert_eq!(
            turns,
            [Turn::Straight, Turn::Straight, Turn::Right],
            "tick records each of your moves, including not steering"
        );
    }

    #[test]
    fn dies() {
        let mut game = game(vec![snake(&[(5, 0), (5, 1)], Direction::North)]);
        game.tick(Input::Up).unwrap();

        assert_eq!(
            (game.parked(), game.world.snake(YOU).is_some()),
            (true, true),
            "tick respawns you at once when you die, parked"
        );
        assert_eq!(
            game.brain.memory.len(),
            1,
            "tick records the move that killed you"
        );
    }

    #[test]
    fn clone_respawns() {
        let mut game = game(vec![snake(&[(8, 8), (8, 9)], Direction::North)]);
        game.world.snakes.push(SnakeState::Respawning(1));
        game.tick(Input::Up).unwrap();

        assert!(
            game.world.snake(1).is_some(),
            "tick respawns clones whose wait is over"
        );
    }

    #[test]
    fn clones_learn() {
        let trained = simulate_clone_meals(true);
        let untrained = simulate_clone_meals(false);

        assert!(
            trained > 3 * untrained,
            "clones that train on a player who chases food eat far more than clones that never \
             train, but ate {trained} to {untrained}"
        );
    }

    ///
    /// Simulates a game and returns how much food its
    /// clones ate.
    ///
    /// A scripted player chases the nearest food for 300
    /// moves. With `training` off, the brain's memory is
    /// wiped before every move so it never trains, which
    /// gives the baseline that trained clones are measured
    /// against.
    ///
    fn simulate_clone_meals(training: bool) -> usize {
        /// Moves played in each game, enough for clones to learn.
        const MOVES: usize = 300;

        let mut game = Game::new(Size::new(40, 30).unwrap(), seeded(1));
        let mut player = seeded(2);
        let mut meals = 0;

        for _ in 0..MOVES {
            if !training {
                game.brain.memory.clear();
            }
            let food = game.world.food.clone();
            game.tick(food_seeker(&game.world, &mut player)).unwrap();

            meals += fed_clones(&game.world, &food);
        }

        meals
    }

    /// Clones whose head is on a cell that held `food` before the move.
    fn fed_clones(world: &World, food: &[Point]) -> usize {
        world
            .snakes
            .iter()
            .skip(1)
            .filter_map(SnakeState::alive)
            .filter(|clone| food.contains(&clone.head))
            .count()
    }
}
