use bingo::Card;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use simulator::aggregation::*;
use simulator::rng_helper::generate;
use std::cell;
use std::io::Write;

thread_local! {
	static RNG:cell::RefCell<ChaCha20Rng> = cell::RefCell::new(generate());
}

fn main() {}

fn play(n: usize) {}
