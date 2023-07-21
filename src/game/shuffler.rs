use derive_more::Constructor;
use rand::{self, seq::SliceRandom, SeedableRng};
use std::cell::RefCell;
// We use Pcg64Mcg over StdRng because it's faster and we don't need cryptographic security.
// Also referencing Pcg64Mcg directly instead of SmallRng since apparently SmallRng can
// decide to change implementations, and it might be nice to have reproducible games
// based on a seed.
// (Although tbh if we're going to serialize games, we might just want to serialize the set
// of cards that got drawn rather than the seed used to pick them)
use rand_pcg::Pcg64Mcg as PRng;

use super::card_pile::CardPile;

pub trait Shuffler<T>: std::fmt::Debug {
    fn shuffle(&self, input: &mut CardPile);
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
    fn shuffle(&self, input: &mut CardPile) {
        let mut rng = self.rng.borrow_mut();
        input.temp_internal_vec().shuffle(&mut *rng);
    }
}

#[derive(Debug, Constructor)]
pub struct NoShuffle;
impl<T> Shuffler<T> for NoShuffle {
    fn shuffle(&self, _input: &mut CardPile) {}
}
