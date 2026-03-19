use crate::error::*;

pub const EDIGE_SIZE: usize = 5;
pub const LINEAR_SIZE: usize = EDIGE_SIZE * EDIGE_SIZE;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
	Row,
	Column,
	MainDiagonal,
	AntiDiagonal,
}

pub struct Iter {
	pub(super) pivot: u8,
	pub(super) remaining: usize,
	pub(super) offset: u8,
}

impl Iter {
	pub(super) fn try_new(linear: usize, direction: Direction) -> Result<Self> {
		if linear >= LINEAR_SIZE {
			Err(Error::LinearOutOfRange(linear))
		} else {
			let linear = match direction {
				Direction::Row => (linear / EDIGE_SIZE) * EDIGE_SIZE,
				Direction::Column => linear % EDIGE_SIZE,
				Direction::MainDiagonal => {
					if linear % 6 != 0 {
						return Err(Error::MainDiagonalCantDefined(linear));
					}

					0
				}
				Direction::AntiDiagonal => {
					if linear == 0 || linear % 4 != 0 {
						return Err(Error::AntiDiagonalCantDefined(linear));
					}

					4
				}
			};

			Ok(Self {
				pivot: linear as u8,
				remaining: EDIGE_SIZE,
				offset: match direction {
					Direction::Row => 1,
					Direction::Column => 5,
					Direction::MainDiagonal => 6,
					Direction::AntiDiagonal => 4,
				},
			})
		}
	}
}

impl Iterator for Iter {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		if self.remaining == 0 {
			None
		} else {
			let ret = self.pivot as usize;
			self.remaining -= 1;
			self.pivot += self.offset;
			Some(ret)
		}
	}
}

impl ExactSizeIterator for Iter {
	fn len(&self) -> usize {
		self.remaining
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::error::*;
	fn top(linear: u8) -> u8 {
		(linear / EDIGE_SIZE as u8) * EDIGE_SIZE as u8
	}

	fn left(linear: u8) -> u8 {
		linear % EDIGE_SIZE as u8
	}

	#[test]
	fn try_new_row() {
		for i in 0..LINEAR_SIZE {
			let fixture = Iter::try_new(i, Direction::Row).unwrap();
			assert_eq!(fixture.pivot, top(i as u8));
			assert_eq!(fixture.remaining, EDIGE_SIZE);
			assert_eq!(fixture.offset, 1);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::Row),
			Err(Error::LinearOutOfRange(x)) if x==LINEAR_SIZE
		));
	}

	#[test]
	fn try_new_col() {
		for i in 0..LINEAR_SIZE {
			let fixture = Iter::try_new(i, Direction::Column).unwrap();
			assert_eq!(fixture.pivot, left(i as u8));
			assert_eq!(fixture.remaining, 5);
			assert_eq!(fixture.offset, 5);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::Column),
			Err(Error::LinearOutOfRange(x)) if x==25
		));
	}

	#[test]
	fn try_new_main_diag() {
		for i in (0..EDIGE_SIZE).map(|i| i * 6) {
			let fixture = Iter::try_new(i, Direction::MainDiagonal).unwrap();
			assert_eq!(fixture.pivot, 0);
			assert_eq!(fixture.remaining, 5);
			assert_eq!(fixture.offset, 6);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::MainDiagonal),
			Err(Error::LinearOutOfRange(x)) if x==25
		));

		assert!(matches!(
			Iter::try_new(4,Direction::MainDiagonal),
			Err(Error::MainDiagonalCantDefined(x)) if x==4
		));
	}

	#[test]
	fn try_new_anti_diag() {
		for i in (1..6).map(|x| 4 * x) {
			let fixture = Iter::try_new(i, Direction::AntiDiagonal).unwrap();
			assert_eq!(fixture.pivot, 4);
			assert_eq!(fixture.remaining, 5);
			assert_eq!(fixture.offset, 4);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::AntiDiagonal),
			Err(Error::LinearOutOfRange(x)) if x==25
		));
		assert!(matches!(
			Iter::try_new(0,Direction::AntiDiagonal),
			Err(Error::AntiDiagonalCantDefined(x)) if x==0
		));

		assert!(matches!(
			Iter::try_new(1,Direction::AntiDiagonal),
			Err(Error::AntiDiagonalCantDefined(x)) if x==1
		));
	}

	#[test]
	fn next() {
		for piv in (0..EDIGE_SIZE).map(|x| x * 5 + 2) {
			let fixture = Iter::try_new(piv, Direction::Row).unwrap();

			for (cnt, act) in fixture.enumerate() {
				assert_eq!(act, piv + cnt - 2);
			}
		}

		for piv in 10..=14 {
			let fixture = Iter::try_new(piv, Direction::Column).unwrap();

			for (cnt, act) in fixture.enumerate() {
				assert_eq!(act, (piv - 10) + (cnt * 5))
			}
		}

		for piv in (0..5).map(|x| x * 6) {
			let fixture = Iter::try_new(piv, Direction::MainDiagonal).unwrap();

			for (cnt, act) in fixture.enumerate() {
				assert_eq!(act, cnt * 6)
			}
		}

		for piv in (0..5).map(|x| 4 + x * 4) {
			let fixture = Iter::try_new(piv, Direction::AntiDiagonal).unwrap();

			for (cnt, act) in fixture.enumerate() {
				assert_eq!(act, (cnt + 1) * 4)
			}
		}
	}

	#[test]
	fn len() {
		let mut fixture = Iter::try_new(0, Direction::Row).unwrap();
		assert_eq!(fixture.len(), 5);

		_ = fixture.next().unwrap();
		assert_eq!(fixture.len(), 4);

		_ = fixture.next().unwrap();
		assert_eq!(fixture.len(), 3);

		_ = fixture.next().unwrap();
		assert_eq!(fixture.len(), 2);

		_ = fixture.next().unwrap();
		assert_eq!(fixture.len(), 1);

		_ = fixture.next().unwrap();
		assert_eq!(fixture.len(), 0);

		_ = fixture.next().is_none();
		assert_eq!(fixture.len(), 0);
	}
}
