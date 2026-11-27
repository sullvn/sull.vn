use std::collections::VecDeque;

use sullgrad::error::LengthError;
use sullgrad::{Model, Sample};

use crate::random::pick;
use crate::senses::Senses;
use crate::tuning::{
    BATCH_LESSONS, HIDDEN_NEURONS, INITIAL_WEIGHT, LEARNING_RATE, MEMORY_LESSONS, MIN_LESSONS,
    TRAIN_STEPS,
};
use crate::turn::Turn;

///
/// The network every clone shares, and the lessons it
/// learns from.
///
/// Every lesson is one of your moves, so the clones
/// learn to move like you.
///
#[derive(Debug)]
pub(crate) struct Brain {
    model: Model,
    pub(crate) memory: VecDeque<Lesson>,
}

/// One of your moves: what you noticed, and which way you went.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Lesson {
    pub(crate) senses: Senses,
    pub(crate) turn: Turn,
}

impl Brain {
    /// A brain with random weights and nothing learned.
    pub(crate) fn new(random: &mut impl FnMut() -> f64) -> Self {
        let inputs = Senses::default().len();
        let outputs = Turn::ALL.len();
        let model = Model::from_fn(inputs, &[HIDDEN_NEURONS, outputs], |_| {
            (random() - 0.5) * (2.0 * INITIAL_WEIGHT)
        });

        Self {
            model,
            memory: VecDeque::with_capacity(MEMORY_LESSONS),
        }
    }

    ///
    /// The move the brain rates highest.
    ///
    /// Always taking the top pick plays far better than
    /// picking in proportion to the ratings.
    ///
    pub(crate) fn choose(&self, senses: &Senses) -> Result<Turn, LengthError> {
        let ratings = self.model.forward(senses)?;
        let top = Turn::ALL
            .into_iter()
            .zip(ratings)
            .max_by(|(_, a), (_, b)| a.total_cmp(b));

        Ok(top.map_or(Turn::Straight, |(turn, _)| turn))
    }

    /// Remembers `lesson`, forgetting the oldest past the limit.
    pub(crate) fn remember(&mut self, lesson: Lesson) {
        if self.memory.len() == MEMORY_LESSONS {
            self.memory.pop_front();
        }
        self.memory.push_back(lesson);
    }

    /// Learns from the lessons in memory, once there are enough.
    pub(crate) fn train(&mut self, random: &mut impl FnMut() -> f64) -> Result<(), LengthError> {
        if self.memory.len() < MIN_LESSONS {
            return Ok(());
        }

        let by_turn = group(&self.memory);
        for _ in 0..TRAIN_STEPS {
            self.model
                .train_step(&batch(&by_turn, random), LEARNING_RATE)?;
        }

        Ok(())
    }
}

/// Lessons grouped by turn, leaving out turns never taken.
fn group(memory: &VecDeque<Lesson>) -> Vec<Vec<&Lesson>> {
    Turn::ALL
        .into_iter()
        .map(|turn| memory.iter().filter(|l| l.turn == turn).collect::<Vec<_>>())
        .filter(|lessons| !lessons.is_empty())
        .collect()
}

///
/// Random lessons with each turn equally likely.
///
/// Most moves go straight. Without the balance, the
/// brain learns little else.
///
fn batch<'a>(by_turn: &[Vec<&'a Lesson>], random: &mut impl FnMut() -> f64) -> Vec<Sample<'a>> {
    (0..BATCH_LESSONS)
        .filter_map(|_| {
            let lessons = pick(random, by_turn)?;
            let lesson = pick(random, lessons)?;

            Some(Sample {
                inputs: &lesson.senses,
                targets: lesson.turn.targets(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{scripted, seeded};

    fn lesson(turn: Turn) -> Lesson {
        Lesson {
            senses: [0.5; 9],
            turn,
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
    fn choose() {
        let mut parameters = [0.0; 211];
        parameters[210] = 1.0;
        let brain = Brain {
            model: Model::from_parameters(9, &[16, 3], &parameters).unwrap(),
            memory: VecDeque::new(),
        };

        assert_eq!(
            brain.choose(&[0.0; 9]),
            Ok(Turn::Right),
            "choose takes the highest rated move"
        );
    }

    #[test]
    fn remember() {
        let mut brain = Brain::new(&mut scripted(&[0.5]));
        brain.remember(lesson(Turn::Left));
        for _ in 0..480 {
            brain.remember(lesson(Turn::Straight));
        }

        assert_eq!(
            (brain.memory.len(), brain.memory.front()),
            (480, Some(&lesson(Turn::Straight))),
            "remember keeps the newest 480 lessons"
        );
    }

    #[test]
    fn batch_balanced() {
        let memory: VecDeque<_> = [Turn::Left, Turn::Straight, Turn::Straight, Turn::Straight]
            .map(lesson)
            .into();
        let by_turn = group(&memory);
        let batch = batch(&by_turn, &mut scripted(&[0.0, 0.0, 0.5, 0.0]));
        let lefts = batch
            .iter()
            .filter(|sample| sample.targets == Turn::Left.targets())
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
            brain.remember(lesson(Turn::Left));
        }
        brain.train(&mut scripted(&[0.5])).unwrap();

        assert_eq!(
            brain.model.parameters(),
            before,
            "train waits for 16 lessons"
        );
    }

    #[test]
    fn learns() {
        let mut brain = Brain::new(&mut seeded(1));
        let mut random = seeded(2);
        for _ in 0..16 {
            brain.remember(lesson(Turn::Left));
        }
        for _ in 0..10 {
            brain.train(&mut random).unwrap();
        }

        assert_eq!(
            brain.choose(&[0.5; 9]),
            Ok(Turn::Left),
            "train teaches the brain the moves in its lessons"
        );
    }
}
