use derive_more::Constructor;
use rand::{self, rngs::StdRng, Rng, SeedableRng};

pub trait Shuffler<T> {
    /**
     * Consumes values from input and returns a shuffled Vec.
     */
    fn shuffle(&mut self, input: &mut Vec<T>) -> Vec<T>;
}

#[derive(Debug)]
pub struct RandomShuffler {
    rng: StdRng,
}
impl RandomShuffler {
    pub fn new(seed: u64) -> Self {
        RandomShuffler {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}
impl<T> Shuffler<T> for RandomShuffler {
    fn shuffle(&mut self, input: &mut Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(input.len());
        while !input.is_empty() {
            let index = self.rng.gen_range(0..input.len());
            result.push(input.remove(index));
        }
        result
    }
}

#[derive(Debug, Constructor)]
pub struct NoShuffle;
impl<T> Shuffler<T> for NoShuffle {
    fn shuffle(&mut self, input: &mut Vec<T>) -> Vec<T> {
        input.drain(..).collect()
    }
}

/**
 * Throws away input and returns cards from test data instead
 */
#[derive(Debug, Constructor)]
pub struct PredestinedShuffler<T> {
    cards: Vec<T>,
}
impl<T> Shuffler<T> for PredestinedShuffler<T> {
    fn shuffle(&mut self, input: &mut Vec<T>) -> Vec<T> {
        self.cards.drain(0..input.len()).collect()
    }
}
