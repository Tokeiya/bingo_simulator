use super::error::Result;
use owo_colors::{OwoColorize, Style};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::{Rng, RngExt};
use std::fmt::{Display, Formatter};
use std::sync::{LazyLock, Mutex};
static UP: LazyLock<Mutex<Style>> =
	LazyLock::new(|| Mutex::new(Style::new().bold().on_bright_white().black()));
static DOWN: LazyLock<Mutex<Style>> = LazyLock::new(|| Mutex::new(Style::new().white()));

const ARRAY: [[i8; 15]; 5] = [
	[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
	[31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
	[46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
	[61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75],
];

pub fn set_up_style(style: Style) {
	let mut g = UP.lock().unwrap();
	*g = style;
}

pub fn set_down_style(style: Style) {
	let mut g = DOWN.lock().unwrap();
	*g = style;
}

#[derive(Debug)]
pub struct Card([i8; 25]);

impl Card {
	pub fn new(rnd: &mut impl Rng) -> Self {
		let mut arr: [i8; 25] = [0; 25];

		for i in 0..5 {
			let candidate = ARRAY[i].sample(rnd, 5);

			for (j, v) in candidate.enumerate() {
				arr[i * 5 + j] = *v;
			}
		}

		Self(arr)
	}

	pub fn up(row: usize, col: usize) -> Result<()> {
		todo!()
	}

	pub fn status(row: usize, col: usize) -> Result<bool> {
		todo!()
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rand_core::TryRng;
	use std::convert::Infallible;
	
	struct Mock;

	impl TryRng for Mock {
		type Error = Infallible;

		fn try_next_u32(&mut self) -> std::result::Result<u32, Self::Error> {
			Ok(0)
		}

		fn try_next_u64(&mut self) -> std::result::Result<u64, Self::Error> {
			Ok(0)
		}

		fn try_fill_bytes(&mut self, dst: &mut [u8]) -> std::result::Result<(), Self::Error> {
			unreachable!()
		}
	}

	#[test]
	fn new_test() {
		let mut mock = Mock;
		let card = Card::new(&mut mock);
		assert_eq!(
			card.0,
			[
				12, 13, 14, 15, 1, 27, 28, 29, 30, 16, 42, 43, 44, 45, 31, 57, 58, 59, 60, 46, 72,
				73, 74, 75, 61
			]
		);
	}
}
