use std::cell::RefCell;

use derive_more::Constructor;
use rand::{self, Rng, SeedableRng};
// We use Pcg64Mcg over StdRng because it's faster and we don't need cryptographic security.
// Also referencing Pcg64Mcg directly instead of SmallRng since apparently SmallRng can
// decide to change implementations, and it might be nice to have reproducible games
// based on a seed.
// (Although tbh if we're going to serialize games, we might just want to serialize the set
// of cards that got drawn rather than the seed used to pick them)
use rand_pcg::Pcg64Mcg as PRng;

pub trait Shuffler<T>: std::fmt::Debug {
    /**
     * Consumes values from input and returns a shuffled Vec.
     */
    fn shuffle(&self, input: &mut Vec<T>) -> Vec<T>;
}

#[derive(Debug)]
pub struct RandomShuffler {
    rng: RefCell<PRng>,
}
impl RandomShuffler {
    #[cfg(test)]
    pub fn new(seed: u64) -> Self {
        RandomShuffler {
            rng: PRng::seed_from_u64(seed).into(),
        }
    }

    pub fn unseeded() -> Self {
        RandomShuffler {
            rng: PRng::from_entropy().into(),
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
