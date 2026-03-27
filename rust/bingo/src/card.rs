use crate::error::*;
use crate::point::Point;
use crate::remaining::Remaining;
use crate::wrapper::ColorizeWrapper;
use rand::prelude::IndexedRandom;
use rand::prelude::Rng;
use std::fmt::{Display, Formatter};

pub const EDGE_SIZE: usize = 5;
pub const LINEAR_SIZE: usize = EDGE_SIZE * EDGE_SIZE;

const BINGO_VALUE: [[i8; 15]; 5] = [
	[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
	[31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],
	[46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
	[61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75],
];

#[derive(Debug)]
pub struct Card {
	storage: [i8; LINEAR_SIZE],
	remaining: Remaining,
}

impl Card {
	pub fn new(rnd: &mut impl Rng, center_free: bool) -> Self {
		let mut arr = [0i8; LINEAR_SIZE];

		let mut idx: usize = 0;

		for r in 0..EDGE_SIZE {
			for num in BINGO_VALUE[r].sample(rnd, EDGE_SIZE) {
				arr[idx] = *num;
				idx += 1;
			}
		}

		let mut remaining = Remaining::new();

		if center_free {
			arr[12] = 0;
			remaining
				.decrement(&Point::try_from_linear(12).unwrap())
				.unwrap();
		}

		Self {
			storage: arr,
			remaining,
		}
	}

	pub fn set(&mut self, value: u8) -> Result<Option<Point>> {
		let rng = if value > 0 && value <= 15 {
			0..5
		} else if value > 15 && value <= 30 {
			5..10
		} else if value > 30 && value <= 45 {
			10..15
		} else if value > 45 && value <= 60 {
			15..20
		} else if value > 60 && value <= 75 {
			20..25
		} else {
			return Err(Error::InvalidCellValue(value));
		};

		for i in rng {
			if self.storage[i] == value as i8 {
				let p = Point::try_from_linear(i as u8).unwrap();
				self.storage[i] *= -1;
				self.remaining.decrement(&p).unwrap();
				return Ok(Some(p));
			}
		}

		Ok(None)
	}

	pub fn get(&self, point: &Point) -> Result<bool> {
		if point.linear() >= LINEAR_SIZE {
			Err(Error::LinearOutOfRange(point.linear()))
		} else {
			Ok(self.storage[point.linear()] <= 0)
		}
	}

	pub fn remaining(&self) -> &Remaining {
		&self.remaining
	}
}

impl Display for Card {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.remaining.view().contains(&0) {
			writeln!(f, "{}", "B  I  N  G  O".bright_white().on_green())?;
		} else {
			writeln!(f, "{}", "B  I  N  G  O".green())?;
		}

		for r in 0..EDGE_SIZE {
			for c in 0..EDGE_SIZE {
				let p = Point::try_from_grid(r as u8, c as u8).unwrap();
				let val = self.storage[p.linear()];

				if val <= 0 {
					write!(f, "{:2}", val.abs().on_bright_white().black().bold())?;
				} else {
					write!(f, "{:2}", val.abs().on_black().bright_white().bold())?;
				}

				write!(f, " ")?;
			}

			writeln!(f)?
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rand::TryRng;
	use std::convert::Infallible;
	
	struct MockRnd;

	impl TryRng for MockRnd {
		type Error = Infallible;

		fn try_next_u32(&mut self) -> std::result::Result<u32, Self::Error> {
			Ok(42)
		}

		fn try_next_u64(&mut self) -> std::result::Result<u64, Self::Error> {
			Ok(42)
		}

		fn try_fill_bytes(&mut self, dst: &mut [u8]) -> std::result::Result<(), Self::Error> {
			unreachable!()
		}
	}

	#[test]
	fn new() {
		let fixture = Card::new(&mut MockRnd, true);
		assert_eq!(
			&fixture.storage,
			&[
				12, 13, 14, 15, 1, 27, 28, 29, 30, 16, 42, 43, 0, 45, 31, 57, 58, 59, 60, 46, 72,
				73, 74, 75, 61
			]
		);
		assert_eq!(
			fixture.remaining.view(),
			&[5, 5, 4, 5, 5, 5, 5, 4, 5, 5, 4, 4]
		);

		let fixture = Card::new(&mut MockRnd, false);
		assert_eq!(
			&fixture.storage,
			&[
				12, 13, 14, 15, 1, 27, 28, 29, 30, 16, 42, 43, 44, 45, 31, 57, 58, 59, 60, 46, 72,
				73, 74, 75, 61
			]
		);
		assert_eq!(fixture.remaining.view(), &[5u8; 12]);
	}

	#[test]
	fn set() {
		let mut fixture = Card {
			storage: std::array::from_fn::<i8, 25, _>(|i| (i + 1) as i8),
			remaining: Remaining::new(),
		};

		fixture.storage[12] = 40;

		assert!(fixture.set(42).unwrap().is_none());
		assert_eq!(fixture.remaining.view(), &[5u8; 12]);

		assert!(matches!(fixture.set(76),Err(Error::InvalidCellValue(x)) if x==76));
		assert_eq!(fixture.remaining.view(), &[5u8; 12]);

		assert!(matches!(fixture.set(40),Ok(Some(x)) if x.linear()==12));
		assert_eq!(
			fixture.remaining.view(),
			&[5, 5, 4, 5, 5, 5, 5, 4, 5, 5, 4, 4]
		);
	}

	#[test]
	fn get() {
		let mut fixture = Card {
			storage: std::array::from_fn::<i8, 25, _>(|i| (i + 1) as i8),
			remaining: Remaining::new(),
		};

		fixture.storage[12] = 0;
		fixture.storage[3] = -4;

		let p = Point::try_from_linear(0).unwrap();
		assert!(!fixture.get(&p).unwrap());

		let p = Point::try_from_linear(12).unwrap();
		assert!(fixture.get(&p).unwrap());

		let p = Point::try_from_linear(3).unwrap();
		assert!(fixture.get(&p).unwrap());
	}

	#[test]
	fn remaining() {
		let mut fixture = Card {
			storage: std::array::from_fn::<i8, 25, _>(|i| (i + 1) as i8),
			remaining: Remaining::new(),
		};

		assert_eq!(fixture.remaining.view(), &[5u8; 12]);
	}

	#[test]
	fn display() {
		let mut fixture = Card {
			storage: std::array::from_fn::<i8, 25, _>(|i| (i + 1) as i8),
			remaining: Remaining::new(),
		};

		fixture.storage[12] = 0;

		for i in 1..=5 {
			fixture.set(i).unwrap();
		}

		dbg!(fixture.remaining.view());

		println!("{}", fixture);
	}

	#[test]
	fn foo() {
		let mut remaining = Remaining::new();
		remaining.decrement(&Point::try_from_linear(12).unwrap());
		let mut fixture = Card {
			storage: [
				11, 6, 1, 13, 3, 30, 21, 16, 19, 28, 43, 35, 0, 39, 45, 57, 58, 47, 60, 59, 61, 62,
				64, 71, 70,
			],
			remaining,
		};

		fixture.set(3).unwrap();
		fixture.set(19).unwrap();
		fixture.set(58).unwrap();
		fixture.set(71).unwrap();
		fixture.set(70).unwrap();

		assert_eq!(
			fixture.remaining.view(),
			&[5, 4, 4, 3, 3, 4, 4, 4, 4, 3, 3, 1]
		);
	}
}
