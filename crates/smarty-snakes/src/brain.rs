use std::collections::VecDeque;

use rand::seq::IteratorRandom;
use rand::{Rng, RngExt};
use sullgrad::error::LengthError;
use sullgrad::{Model, Sample};

use crate::senses::Senses;
use crate::tuning::{
    BATCH_SAMPLES, HIDDEN_NEURONS, INITIAL_WEIGHT, LEARNING_RATE, MAX_SAMPLES, MIN_SAMPLES,
    TRAIN_STEPS,
};
use crate::turn::Turn;

///
/// The network every bot shares, and the samples it
/// learns from.
///
/// Every sample is one of the player's moves, so the
/// bots learn to move like the player.
///
#[derive(Debug)]
pub(crate) struct Brain {
    model: Model,
    pub(crate) samples: VecDeque<PlayerSample>,
}

/// One of the player's moves: what they sensed, and which way they turned
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PlayerSample {
    pub(crate) inputs: Senses,
    pub(crate) target: Turn,
}

impl Brain {
    /// A brain with random weights and nothing learned
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        let inputs = Senses::default().len();
        let outputs = Turn::ALL.len();
        let model = Model::from_fn(inputs, &[HIDDEN_NEURONS, outputs], |_| {
            rng.random_range(-INITIAL_WEIGHT..INITIAL_WEIGHT)
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

    /// Records `sample`, dropping the oldest past the limit
    pub(crate) fn record(&mut self, sample: PlayerSample) {
        if self.samples.len() == MAX_SAMPLES {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }

    /// Learns from the recorded samples, once there are enough
    pub(crate) fn train(&mut self, rng: &mut impl Rng) -> Result<(), LengthError> {
        if self.samples.len() < MIN_SAMPLES {
            return Ok(());
        }

        for _ in 0..TRAIN_STEPS {
            self.model
                .train_step(&batch(&self.samples, rng), LEARNING_RATE)?;
        }

        Ok(())
    }
}

/// Random samples for one training step
fn batch<'a>(samples: &'a VecDeque<PlayerSample>, rng: &mut impl Rng) -> Vec<Sample<'a>> {
    (0..BATCH_SAMPLES)
        .filter_map(|_| balanced_sample(samples, rng))
        .collect()
}

///
/// A random sample, with each turn the player has made
/// equally likely.
///
/// Most moves go straight. Without the balance, the
/// brain learns little else.
///
/// Drawing from every sample also beats taking the
/// newest. In simulations, bots trained on the newest
/// ate about 7% less.
///
fn balanced_sample<'a>(
    samples: &'a VecDeque<PlayerSample>,
    rng: &mut impl Rng,
) -> Option<Sample<'a>> {
    let with_target = |turn: Turn| samples.iter().filter(move |sample| sample.target == turn);
    let turn = Turn::ALL
        .into_iter()
        .filter(|&turn| with_target(turn).next().is_some())
        .choose(rng)?;
    let sample = with_target(turn).choose(rng)?;

    Some(Sample {
        inputs: &sample.inputs,
        targets: turn.targets(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::seeded;

    fn sample(turn: Turn) -> PlayerSample {
        PlayerSample {
            inputs: [0.5; 9],
            target: turn,
        }
    }

    #[test]
    fn new() {
        let brain = Brain::new(&mut seeded(1));
        let parameters = brain.model.parameters();

        assert!(
            parameters.iter().all(|weight| (-0.3..0.3).contains(weight)),
            "new starts weights from -0.3 to 0.3"
        );
        assert_ne!(
            parameters.first(),
            parameters.last(),
            "new starts weights at different values"
        );
        assert_eq!(
            parameters.len(),
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
        let mut brain = Brain::new(&mut seeded(1));
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
        let mut rng = seeded(1);
        let picked: Vec<_> = (0..100).flat_map(|_| batch(&samples, &mut rng)).collect();
        let lefts = picked
            .iter()
            .filter(|sample| sample.targets == Turn::Left.targets())
            .count();

        assert_eq!(picked.len(), 1600, "batch picks 16 samples");
        assert!(
            (700..900).contains(&lefts),
            "batch picks each turn taken about equally often, however rare, \
             but picked left {lefts} times in 1600"
        );
    }

    #[test]
    fn train_waits() {
        let mut brain = Brain::new(&mut seeded(1));
        let before = brain.model.parameters().to_vec();
        for _ in 0..15 {
            brain.record(sample(Turn::Left));
        }
        brain.train(&mut seeded(2)).unwrap();

        assert_eq!(
            brain.model.parameters(),
            before,
            "train waits for 16 samples"
        );
    }

    #[test]
    fn learns() {
        let mut brain = Brain::new(&mut seeded(1));
        let mut rng = seeded(2);
        for _ in 0..16 {
            brain.record(sample(Turn::Left));
        }
        for _ in 0..10 {
            brain.train(&mut rng).unwrap();
        }

        assert_eq!(
            brain.predict(&[0.5; 9]),
            Ok(Turn::Left),
            "train teaches the brain the moves in its samples"
        );
    }
}
