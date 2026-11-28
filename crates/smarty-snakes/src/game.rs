use sullgrad::error::LengthError;

use crate::brain::{Brain, PlayerSample};
use crate::controls::{Input, steer, wake};
use crate::senses::sense;
use crate::turn::Turn;
use crate::world::{Direction, PLAYER, Size, SnakeState, World};

///
/// Snakes on a shared board, where every bot learns
/// from the player.
///
/// The world only moves while the player does. They
/// start parked, and park again whenever they die.
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
    /// A new game, with the player parked.
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

    /// Whether the player is parked, so nothing moves.
    pub(crate) const fn parked(&self) -> bool {
        self.parked
    }

    /// The board as one cell code per cell, row by row.
    pub(crate) fn cells(&self) -> Box<[u8]> {
        self.world.cells()
    }

    ///
    /// Plays one move, with `input` steering the player.
    ///
    /// Does nothing if the player is parked and `input`
    /// doesn't wake them.
    ///
    pub(crate) fn tick(&mut self, input: Input) -> Result<(), LengthError> {
        let Some(&player) = self.world.snake(PLAYER) else {
            self.revive_player();
            return Ok(());
        };
        let Some(player_turn) = self.player_turn(input, player.heading) else {
            return Ok(());
        };

        self.world.count_down(&mut self.random);
        self.brain.record(PlayerSample {
            inputs: sense(&self.world, &player),
            target: player_turn,
        });

        let turns = self.turns(player_turn)?;
        self.world.step(&turns, &mut self.random);
        if self.world.snake(PLAYER).is_none() {
            self.revive_player();
        }

        self.brain.train(&mut self.random)
    }

    ///
    /// The player's turn for `input`, or `None` if they
    /// stay parked.
    ///
    /// A turn that wakes them ends the parking.
    ///
    fn player_turn(&mut self, input: Input, heading: Direction) -> Option<Turn> {
        if !self.parked {
            return Some(steer(input, heading));
        }
        let turn = wake(input, heading)?;
        self.parked = false;

        Some(turn)
    }

    /// Respawns the player, parked until they next steer.
    fn revive_player(&mut self) {
        self.world.respawn(PLAYER, &mut self.random);
        self.parked = true;
    }

    /// Each snake's move in order, with `None` for the dead.
    fn turns(&self, player_turn: Turn) -> Result<Vec<Option<Turn>>, LengthError> {
        self.world
            .snakes
            .iter()
            .enumerate()
            .map(|(index, state)| self.turn_for(index, state, player_turn))
            .collect()
    }

    ///
    /// The move snake `index` makes, or `None` if it's dead.
    ///
    /// The player makes `player_turn`. A bot makes the
    /// turn the brain predicts.
    ///
    fn turn_for(
        &self,
        index: usize,
        state: &SnakeState,
        player_turn: Turn,
    ) -> Result<Option<Turn>, LengthError> {
        let Some(snake) = state.alive() else {
            return Ok(None);
        };
        if index == PLAYER {
            return Ok(Some(player_turn));
        }

        self.brain.predict(&sense(&self.world, snake)).map(Some)
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
            (game.parked(), game.cells(), game.brain.samples.len()),
            (true, before, 0),
            "tick does nothing while parked, without an input that wakes the player"
        );
    }

    #[test]
    fn wakes() {
        let mut game = game(vec![snake(&[(5, 5), (5, 6)], Direction::North)]);
        game.tick(Input::Left).unwrap();

        assert_eq!(
            (game.parked(), head(&game, PLAYER)),
            (false, Some(Point { x: 4, y: 5 })),
            "tick wakes the player with an arrow, and makes that move"
        );
    }

    #[test]
    fn tap_wakes() {
        let mut game = game(vec![snake(&[(5, 5), (5, 6)], Direction::North)]);
        game.tick(Input::TapLeft).unwrap();

        assert_eq!(
            head(&game, PLAYER),
            Some(Point { x: 5, y: 4 }),
            "tick wakes the player with a tap, which sets off straight"
        );
    }

    #[test]
    fn bots_move() {
        let mut game = game(vec![
            snake(&[(5, 5), (5, 6)], Direction::North),
            snake(&[(2, 2), (2, 3)], Direction::North),
        ]);
        game.tick(Input::Up).unwrap();

        assert_ne!(
            head(&game, 1),
            Some(Point { x: 2, y: 2 }),
            "tick moves bots as the player moves"
        );
    }

    #[test]
    fn samples() {
        let mut game = game(vec![snake(&[(5, 5), (5, 6)], Direction::North)]);
        game.tick(Input::Up).unwrap();
        game.tick(Input::None).unwrap();
        game.tick(Input::Right).unwrap();
        let turns: Vec<_> = game
            .brain
            .samples
            .iter()
            .map(|sample| sample.target)
            .collect();

        assert_eq!(
            turns,
            [Turn::Straight, Turn::Straight, Turn::Right],
            "tick records each of the player's moves, including not steering"
        );
    }

    #[test]
    fn dies() {
        let mut game = game(vec![snake(&[(5, 0), (5, 1)], Direction::North)]);
        game.tick(Input::Up).unwrap();

        assert_eq!(
            (game.parked(), game.world.snake(PLAYER).is_some()),
            (true, true),
            "tick respawns the player at once when they die, parked"
        );
        assert_eq!(
            game.brain.samples.len(),
            1,
            "tick records the move that killed the player"
        );
    }

    #[test]
    fn bot_respawns() {
        let mut game = game(vec![snake(&[(8, 8), (8, 9)], Direction::North)]);
        game.world.snakes.push(SnakeState::Respawning(1));
        game.tick(Input::Up).unwrap();

        assert!(
            game.world.snake(1).is_some(),
            "tick respawns bots whose wait is over"
        );
    }

    #[test]
    fn bots_learn() {
        let trained = simulate_bot_meals(true);
        let untrained = simulate_bot_meals(false);

        assert!(
            trained > 3 * untrained,
            "bots that train on a player who chases food eat far more than bots that never \
             train, but ate {trained} to {untrained}"
        );
    }

    ///
    /// Simulates a game and returns how much food its
    /// bots ate.
    ///
    /// A scripted player chases the nearest food for 300
    /// moves. With `training` off, the brain's samples are
    /// wiped before every move so it never trains, which
    /// gives the baseline that trained bots are measured
    /// against.
    ///
    fn simulate_bot_meals(training: bool) -> usize {
        /// Moves played in each game, enough for bots to learn.
        const MOVES: usize = 300;

        let mut game = Game::new(Size::new(40, 30).unwrap(), seeded(1));
        let mut player = seeded(2);
        let mut meals = 0;

        for _ in 0..MOVES {
            if !training {
                game.brain.samples.clear();
            }
            let food = game.world.food.clone();
            game.tick(food_seeker(&game.world, &mut player)).unwrap();

            meals += fed_bots(&game.world, &food);
        }

        meals
    }

    /// Bots whose head is on a cell that held `food` before the move.
    fn fed_bots(world: &World, food: &[Point]) -> usize {
        world
            .snakes
            .iter()
            .skip(1)
            .filter_map(SnakeState::alive)
            .filter(|bot| food.contains(&bot.head))
            .count()
    }
}
