use super::error::Result;
use rand_core::Rng;
use std::fmt::Display;

#[derive(Debug)]
pub struct Card([i8; 25]);

impl Card {
	pub fn new(rnd: &mut impl Rng) -> Self {
		todo!()
	}

	pub fn set(&mut self, row: usize, col: usize) -> Result<()> {
		todo!()
	}

	pub fn get(&self, row: usize, col: usize) -> Result<bool> {
		todo!()
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
