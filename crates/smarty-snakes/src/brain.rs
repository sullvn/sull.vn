use std::collections::VecDeque;

use sullgrad::error::LengthError;
use sullgrad::{Model, Sample};

use crate::random::pick;
use crate::senses::Senses;
use crate::tuning::{
    BATCH_SAMPLES, HIDDEN_NEURONS, INITIAL_WEIGHT, LEARNING_RATE, MAX_SAMPLES, MIN_SAMPLES,
    TRAIN_STEPS,
};
use crate::turn::Turn;

///
/// The network every clone shares, and the samples it
/// learns from.
///
/// Every sample is one of your moves, so the clones
/// learn to move like you.
///
#[derive(Debug)]
pub(crate) struct Brain {
    model: Model,
    pub(crate) samples: VecDeque<PlayerSample>,
}

/// One of the player's moves: what they sensed, and which way they turned.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PlayerSample {
    pub(crate) inputs: Senses,
    pub(crate) target: Turn,
}

impl Brain {
    /// A brain with random weights and nothing learned.
    pub(crate) fn new(random: &mut impl FnMut() -> f64) -> Self {
        let inputs = Senses::default().len();
        let outputs = Turn::ALL.len();
        let model = Model::from_fn(inputs, &[HIDDEN_NEURONS, outputs], |_| {
            initial_weight(random)
        });

        Self {
            model,
            samples: VecDeque::with_capacity(MAX_SAMPLES),
        }
    }

    ///
    /// The turn the brain predicts the player would make,
    /// given `inputs`.
    ///
    /// That's the turn with the highest output. Always
    /// taking the top one plays far better than picking in
    /// proportion to the outputs.
    ///
    pub(crate) fn predict(&self, inputs: &Senses) -> Result<Turn, LengthError> {
        let outputs = self.model.forward(inputs)?;
        let top = Turn::ALL
            .into_iter()
            .zip(outputs)
            .max_by(|(_, a), (_, b)| a.total_cmp(b));

        Ok(top.map_or(Turn::Straight, |(turn, _)| turn))
    }

    /// Records `sample`, dropping the oldest past the limit.
    pub(crate) fn record(&mut self, sample: PlayerSample) {
        if self.samples.len() == MAX_SAMPLES {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }

    /// Learns from the recorded samples, once there are enough.
    pub(crate) fn train(&mut self, random: &mut impl FnMut() -> f64) -> Result<(), LengthError> {
        if self.samples.len() < MIN_SAMPLES {
            return Ok(());
        }

        let by_turn = group(&self.samples);
        for _ in 0..TRAIN_STEPS {
            self.model
                .train_step(&batch(&by_turn, random), LEARNING_RATE)?;
        }

        Ok(())
    }
}

/// A random starting weight, from -[`INITIAL_WEIGHT`] to [`INITIAL_WEIGHT`].
fn initial_weight(random: &mut impl FnMut() -> f64) -> f64 {
    let centered = random() - 0.5;

    centered * (2.0 * INITIAL_WEIGHT)
}

/// Samples grouped by turn, leaving out turns never taken.
fn group(samples: &VecDeque<PlayerSample>) -> Vec<Vec<&PlayerSample>> {
    Turn::ALL
        .into_iter()
        .map(|turn| {
            samples
                .iter()
                .filter(|sample| sample.target == turn)
                .collect::<Vec<_>>()
        })
        .filter(|group| !group.is_empty())
        .collect()
}

///
/// Random samples with each turn equally likely.
///
/// Most moves go straight. Without the balance, the
/// brain learns little else.
///
fn batch<'a>(
    by_turn: &[Vec<&'a PlayerSample>],
    random: &mut impl FnMut() -> f64,
) -> Vec<Sample<'a>> {
    (0..BATCH_SAMPLES)
        .filter_map(|_| {
            let samples = pick(random, by_turn.iter())?;
            let sample = pick(random, samples.iter())?;

            Some(Sample {
                inputs: &sample.inputs,
                targets: sample.target.targets(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{scripted, seeded};

    fn sample(turn: Turn) -> PlayerSample {
        PlayerSample {
            inputs: [0.5; 9],
            target: turn,
        }
    }

    #[test]
    fn new() {
        let brain = Brain::new(&mut scripted(&[0.0, 1.0]));

        assert_eq!(
            brain.model.parameters().get(..2),
            Some([-0.3, 0.3].as_slice()),
            "new starts weights from -0.3 to 0.3"
        );
        assert_eq!(
            brain.model.parameters().len(),
            211,
            "new has 9 inputs, 16 hidden neurons, and 3 outputs"
        );
    }

    #[test]
    fn predict() {
        let mut parameters = [0.0; 211];
        parameters[210] = 1.0;
        let brain = Brain {
            model: Model::from_parameters(9, &[16, 3], &parameters).unwrap(),
            samples: VecDeque::new(),
        };

        assert_eq!(
            brain.predict(&[0.0; 9]),
            Ok(Turn::Right),
            "predict takes the turn with the highest output"
        );
    }

    #[test]
    fn record() {
        let mut brain = Brain::new(&mut scripted(&[0.5]));
        brain.record(sample(Turn::Left));
        for _ in 0..480 {
            brain.record(sample(Turn::Straight));
        }

        assert_eq!(
            (brain.samples.len(), brain.samples.front()),
            (480, Some(&sample(Turn::Straight))),
            "record keeps the newest 480 samples"
        );
    }

    #[test]
    fn batch_balanced() {
        let samples: VecDeque<_> = [Turn::Left, Turn::Straight, Turn::Straight, Turn::Straight]
            .map(sample)
            .into();
        let by_turn = group(&samples);
        let batch = batch(&by_turn, &mut scripted(&[0.0, 0.0, 0.5, 0.0]));
        let lefts = batch
            .iter()
            .filter(|picked| picked.targets == Turn::Left.targets())
            .count();

        assert_eq!(
            (batch.len(), lefts),
            (16, 8),
            "batch picks each turn taken equally often, however rare"
        );
    }

    #[test]
    fn train_waits() {
        let mut brain = Brain::new(&mut scripted(&[0.5, 0.7]));
        let before = brain.model.parameters().to_vec();
        for _ in 0..15 {
            brain.record(sample(Turn::Left));
        }
        brain.train(&mut scripted(&[0.5])).unwrap();

        assert_eq!(
            brain.model.parameters(),
            before,
            "train waits for 16 samples"
        );
    }

    #[test]
    fn learns() {
        let mut brain = Brain::new(&mut seeded(1));
        let mut random = seeded(2);
        for _ in 0..16 {
            brain.record(sample(Turn::Left));
        }
        for _ in 0..10 {
            brain.train(&mut random).unwrap();
        }

        assert_eq!(
            brain.predict(&[0.5; 9]),
            Ok(Turn::Left),
            "train teaches the brain the moves in its samples"
        );
    }
}
