use std::cell::RefCell;

use derive_more::Constructor;
use rand::{self, rngs::StdRng, Rng, SeedableRng};

pub trait Shuffler<T>: std::fmt::Debug {
    /**
     * Consumes values from input and returns a shuffled Vec.
     */
    fn shuffle(&self, input: &mut Vec<T>) -> Vec<T>;
}

#[derive(Debug)]
pub struct RandomShuffler {
    rng: RefCell<StdRng>,
}
impl RandomShuffler {
    #[cfg(test)]
    pub fn new(seed: u64) -> Self {
        RandomShuffler {
            rng: StdRng::seed_from_u64(seed).into(),
        }
    }

    pub fn unseeded() -> Self {
        RandomShuffler {
            rng: StdRng::from_entropy().into(),
        }
    }
}
impl<T> Shuffler<T> for RandomShuffler {
    fn shuffle(&self, input: &mut Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(input.len());
        let mut rng = self.rng.borrow_mut();
        while !input.is_empty() {
            let index = rng.gen_range(0..input.len());
            result.push(input.remove(index));
        }
        result
    }
}

#[derive(Debug, Constructor)]
pub struct NoShuffle;
impl<T> Shuffler<T> for NoShuffle {
    fn shuffle(&self, input: &mut Vec<T>) -> Vec<T> {
        input.drain(..).collect()
    }
}

/**
 * Throws away input and returns cards from test data instead
 */
#[derive(Debug)]
pub struct PredestinedShuffler<T> {
    cards: RefCell<Vec<T>>,
}
impl<T> Shuffler<T> for PredestinedShuffler<T>
where
    T: std::fmt::Debug,
{
    fn shuffle(&self, input: &mut Vec<T>) -> Vec<T> {
        self.cards.borrow_mut().drain(0..input.len()).collect()
    }
}
