use super::error::{Error, Result};
use super::wrapper::ColorizeWrapper;
use rand::prelude::IndexedRandom;
use rand::rand_core::Rng;
use std::borrow::Cow;
use std::fmt::Display;

#[derive(Debug)]
pub struct Card([i8; 25]);

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
			let candidate = SOURCE[i].sample(rnd, 5);

			for (j, v) in candidate.enumerate() {
				arr[i * 5 + j] = *v;
			}
		}

		arr[12] = 0;

		Self(arr)
	}

	fn conv(row: usize, col: usize) -> Result<usize> {
		if row >= 5 {
			Err(Error::RowOutOfRange(row))
		} else if col >= 5 {
			Err(Error::ColumnOutOfRange(col))
		} else {
			Ok(row + col * 5)
		}
	}

	pub fn up(&mut self, row: usize, col: usize) -> Result<i8> {
		let idx = Self::conv(row, col)?;
		if self.0[idx] <= 0 {
			Err(Error::AlreadyUpped(row, col))
		} else {
			self.0[idx] = -self.0[idx];
			Ok(self.0[idx].abs())
		}
	}

	pub fn set(&mut self, num: u8) -> Result<bool> {
		todo!()
	}

	pub fn get(&self, row: usize, col: usize) -> Result<(i8, bool)> {
		let idx = Self::conv(row, col)?;
		Ok((self.0[idx].abs(), self.0[idx] <= 0))
	}

	pub fn check(&self) -> bool {
		let mut flg = true;

		for piv in (0..5).map(|x| x * 5) {
			for idx in piv..piv + 5 {
				if self.0[idx] > 0 {
					flg = false;
					break;
				}
			}

			if flg {
				return true;
			} else {
				flg = true;
			}
		}

		flg = true;

		for piv in 0..5 {
			for idx in (0..5).map(|i| piv + i * 5) {
				if self.0[idx] > 0 {
					flg = false;
					break;
				}
			}

			if flg {
				return true;
			} else {
				flg = true;
			}
		}

		flg = true;

		for idx in (0..5).map(|i| i * 6) {
			if self.0[idx] > 0 {
				flg = false;
				break;
			}
		}

		if flg {
			return true;
		} else {
			flg = true;
		}

		let mut idx = 4usize;
		dbg!(idx);
		for _ in 0..5 {
			if self.0[idx] > 0 {
				return false;
			}

			idx += 4;
		}

		return true;
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fn conv(scr: &str) -> Cow<'_, str> {
			if scr.len() == 1 {
				let mut s = String::new();
				s.push('0');
				s.push_str(scr);
				return Cow::Owned(s);
			} else {
				return Cow::Borrowed(scr);
			}
		}

		writeln!(f, "{}", "B  I  N  G  O".bold())?;

		for r in 0..5 {
			for c in 0..5 {
				let (i, b) = self.get(r, c).unwrap();

				if b {
					write!(f, "{} ", conv(&i.to_string()).on_green().black().bold())?
				} else {
					write!(f, "{} ", conv(&i.to_string()).on_yellow().black().bold())?
				}
			}

			writeln!(f)?
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
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
		assert_eq!(
			fixture.0,
			[
				12, 13, 14, 15, 1, 27, 28, 29, 30, 16, 42, 43, 0, 45, 31, 57, 58, 59, 60, 46, 72,
				73, 74, 75, 61
			]
		);

		let mut rnd = rand::rng();

		for _ in 0..100 {
			let fixture = Card::new(&mut rnd);

			assert!(fixture.0.iter().take(5).all(|x| x <= &15 && x > &0));
			assert!(
				fixture
					.0
					.iter()
					.skip(5)
					.take(5)
					.all(|x| x <= &30 && x > &15)
			);

			assert!(
				fixture
					.0
					.iter()
					.skip(10)
					.take(2)
					.all(|x| x <= &45 && x > &30)
			);

			assert_eq!(fixture.0[12], 0);

			assert!(
				fixture
					.0
					.iter()
					.skip(13)
					.take(2)
					.all(|x| x <= &45 && x > &30)
			);

			assert!(
				fixture
					.0
					.iter()
					.skip(15)
					.take(5)
					.all(|x| x <= &60 && x > &45)
			);

			assert!(fixture.0.iter().skip(20).all(|x| x <= &75 && x > &60));
		}
	}

	#[test]
	fn up() {
		let mut fixture = Card::new(&mut Dummy);

		for c in 0..5 {
			for r in 0..5 {
				println!("Setting card at ({}, {})", r, c);
				if r == 2 && c == 2 {
					assert_eq!(fixture.0[convert(r, c)], 0);
					continue;
				}

				assert!(fixture.0[convert(r, c)] > 0);
				let act = fixture.up(r, c).unwrap();
				assert_eq!(act, fixture.0[convert(r, c)].abs());
				assert!(fixture.0[convert(r, c)] < 0);
			}
		}

		assert!(matches!(fixture.up(5,0),Err(Error::RowOutOfRange(r)) if r==5));
		assert!(matches!(fixture.up(0,5),Err(Error::ColumnOutOfRange(c)) if c==5));

		for c in 0..5 {
			for r in 0..5 {
				assert!(fixture.0[convert(r, c)] <= 0);
				assert!(fixture.up(r, c).is_err());
			}
		}
	}

	#[test]
	fn get() {
		let mut fixture = Card::new(&mut Dummy);

		assert!(matches!(fixture.get(5,0),Err(Error::RowOutOfRange(r)) if r==5));
		assert!(matches!(fixture.get(0,5),Err(Error::ColumnOutOfRange(c)) if c==5));

		for c in 0..5 {
			for r in 0..5 {
				if r == 2 && c == 2 {
					assert!(matches!( fixture.get(r, c), Ok((i,b)) if (i,b)==(0,true)));
				} else {
					assert!(
						matches!(fixture.get(r,c),Ok((i,b)) if (i,b)==(fixture.0[convert(r,c)],false))
					);
					fixture.up(r, c).unwrap();

					assert!(
						matches!(fixture.get(r,c),Ok((i,b)) if (i,b)==(fixture.0[convert(r,c)].abs(),true))
					);
				}
			}
		}
	}

	#[test]
	fn check() {
		fn reset(scr: &mut [i8]) {
			for i in scr.iter_mut() {
				*i = i.abs();
			}

			scr[12] = 0;
		}

		let mut fixture = Card::new(&mut Dummy);

		reset(&mut fixture.0);
		fixture.0[4] *= -1;
		fixture.0[9] *= -1;
		let a = fixture.check();
		dbg!(a);

		assert!(!fixture.check());

		for offset in (0..5).map(|i| i * 5) {
			reset(&mut fixture.0);
			for i in (offset..(offset + 5)) {
				assert!(!fixture.check());
				fixture.0[i] *= -1;
			}
			assert!(fixture.check());
		}

		for piv in 0..5 {
			reset(&mut fixture.0);
			let mut idx = piv;

			for _ in 0..5 {
				println!("({}):{}", idx, fixture.check());
				assert!(!fixture.check());
				fixture.0[idx] *= -1;
				idx += 5;
			}
			assert!(fixture.check());
			println!("------");
		}

		reset(&mut fixture.0);

		for i in [0, 6, 12, 18, 24] {
			assert!(!fixture.check());
			fixture.0[i] *= -1;
		}
		assert!(fixture.check());

		reset(&mut fixture.0);

		for i in [4, 8, 12, 16, 20] {
			assert!(!fixture.check());
			fixture.0[i] *= -1;
		}

		assert!(fixture.check());
	}

	#[test]
	fn set() {
		let mut fixture = Card::new(&mut Dummy);

		fixture.set(12).unwrap();
	}

	#[test]
	fn foo() {
		let fixture = Card::new(&mut Dummy);

		println!("{fixture}")
	}
}
