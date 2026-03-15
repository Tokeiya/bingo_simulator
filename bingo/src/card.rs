use super::error::Result;
use rand::rand_core::Rng;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Card([i8; 25]);

#[derive(Debug)]
pub struct SimpleView<'a>(&'a Card);

const SOURCE: [[i8; 15]; 5] = [
	[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
	[31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
	[46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
	[61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75],
];

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

	pub fn set(&mut self, row: usize, col: usize) -> Result<i8> {
		todo!()
	}

	pub fn get(&self, row: usize, col: usize) -> Result<(i8, bool)> {
		todo!()
	}

	pub fn check(&self) -> bool {
		todo!()
	}

	pub fn simple_view(&self) -> SimpleView<'_> {
		todo!()
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for SimpleView<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::prelude::IteratorRandom;
	use rand::TryRng;
	use std::convert::Infallible;
	pub struct Dummy;

	impl TryRng for Dummy {
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

	fn convert(row: usize, col: usize) -> usize {
		row + col * 5
	}

	#[test]
	fn new() {
		let fixture = Card::new(&mut Dummy);
		todo!()
	}

	#[test]
	fn set() {
		let mut fixture = Card::new(&mut Dummy);

		for c in 0..5 {
			for r in 0..5 {
				if r == 2 && c == 2 {
					assert_eq!(fixture.0[convert(r, c)], 0);
					continue;
				}

				assert!(fixture.0[convert(r, c)] > 0);
				let act = fixture.set(r, c).unwrap();
				assert_eq!(act, fixture.0[convert(r, c)].abs());
				assert!(fixture.0[convert(r, c)] < 0);
			}
		}

		assert!(fixture.get(5, 5).is_err());

		for c in 0..5 {
			for r in 0..5 {
				assert!(fixture.0[convert(r, c)] <= 0);
				assert!(fixture.set(r, c).is_err());
			}
		}
	}

	#[test]
	fn get() {
		let mut fixture = Card::new(&mut Dummy);

		for c in 0..5 {
			for r in 0..5 {
				if r == 2 && c == 2 {
					assert!(matches!( fixture.get(r, c), Ok((i,b)) if (i,b)==(0,true)));
				} else {
					assert!(
						matches!(fixture.get(r,c),Ok((i,b)) if (i,b)==(fixture.0[convert(r,c)],false))
					);
					fixture.set(r, c).unwrap();
					assert!(
						matches!(fixture.get(r,c),Ok((i,b)) if (i,b)==(fixture.0[convert(r,c)],true))
					);
				}
			}
		}
	}

	#[test]
	fn foo() {
		let vec = (1..=15).collect::<Vec<i8>>();
		let mut rng = Dummy;
		let a = vec.iter().sample(&mut rng, 5);

		for i in a.into_iter() {
			println!("{}", i);
		}
	}
}
