//! Numbers that shape how the game plays and learns

/// Board cells for each snake, counting the player's.
pub(crate) const CELLS_PER_SNAKE: i32 = 200;

/// Fewest snakes on a board: the player's and one bot.
pub(crate) const MIN_SNAKES: usize = 2;

/// Length of a newly spawned snake.
pub(crate) const SPAWN_LENGTH: i32 = 3;

///
/// Fewest cells between a spawning head and any edge.
///
/// Leaves room to turn away from the wall.
///
pub(crate) const SPAWN_MARGIN: i32 = 4;

/// Moves a snake can make without eating before it starves.
pub(crate) const STARVATION_MOVES: u32 = 150;

/// Moves a dead bot waits before it reappears.
pub(crate) const RESPAWN_MOVES: u32 = 4;

///
/// Most recent samples kept.
///
/// A minute of play at eight moves per second.
///
pub(crate) const MAX_SAMPLES: usize = 480;

/// Samples needed before training starts.
pub(crate) const MIN_SAMPLES: usize = 16;

///
/// Training steps after each move.
///
/// Training is limited by samples, not steps. In
/// simulations, 32 steps learned no better than 4.
///
pub(crate) const TRAIN_STEPS: usize = 4;

/// Samples in each training step's batch.
pub(crate) const BATCH_SAMPLES: usize = 16;

/// How far each training step moves the weights.
pub(crate) const LEARNING_RATE: f64 = 0.1;

/// Largest starting weight or bias, either side of zero.
pub(crate) const INITIAL_WEIGHT: f64 = 0.3;

/// Neurons in the brain's one hidden layer.
pub(crate) const HIDDEN_NEURONS: usize = 16;

///
/// Cells at which a distance sense reaches its limit.
///
/// Fixed rather than scaled to the board, so the brain
/// works the same on any board size.
///
pub(crate) const SENSE_RANGE: f64 = 10.0;
