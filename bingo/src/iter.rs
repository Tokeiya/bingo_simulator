use crate::error;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
	Row,
	Column,
	MainDiagonal,
	AntiDiagonal,
}

pub struct Iter {
	pub(super) pivot: u8,
	pub(super) remaining: u8,
	pub(super) direction: Direction,
}

impl Iter {
	pub(super) fn try_new(linear: u8, direction: Direction) -> error::Result<Self> {
		todo!()
	}
}

impl Iterator for Iter {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

impl ExactSizeIterator for Iter {
	fn len(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::error::*;
	fn top(linear: u8) -> u8 {
		linear / 5
	}

	fn left(linear: u8) -> u8 {
		linear % 5
	}

	#[test]
	fn try_new_row() {
		for i in 0..25 {
			let fixture = Iter::try_new(i, Direction::Row).unwrap();
			assert_eq!(fixture.pivot, top(i));
			assert_eq!(fixture.remaining, 5);
			assert_eq!(fixture.direction, Direction::Row);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::Row),
			Err(Error::LinearOutOfRange(x)) if x==25
		));
	}

	#[test]
	fn try_new_col() {
		for i in 0..25 {
			let fixture = Iter::try_new(i, Direction::Column).unwrap();
			assert_eq!(fixture.pivot, left(i));
			assert_eq!(fixture.remaining, 5);
			assert_eq!(fixture.direction, Direction::Column);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::Column),
			Err(Error::LinearOutOfRange(x)) if x==25
		));
	}

	#[test]
	fn try_new_main_diag() {
		for i in (0..5).map(|i| i * 6) {
			let fixture = Iter::try_new(i, Direction::MainDiagonal).unwrap();
			assert_eq!(fixture.pivot, 0);
			assert_eq!(fixture.remaining, 5);
			assert_eq!(fixture.direction, Direction::MainDiagonal);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::MainDiagonal),
			Err(Error::LinearOutOfRange(x)) if x==30
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
			assert_eq!(fixture.direction, Direction::AntiDiagonal);
		}

		assert!(matches!(
			Iter::try_new(25, Direction::AntiDiagonal),
			Err(Error::LinearOutOfRange(x)) if x==30
		));

		assert!(matches!(
			Iter::try_new(0,Direction::AntiDiagonal),
			Err(Error::MainDiagonalCantDefined(x)) if x==0
		));
	}

	#[test]
	fn next() {
		for piv in (0u8..5).map(|x| x * 5u8 + 2u8) {
			let mut fixture = Iter::try_new(piv, Direction::Row).unwrap();

			for (cnt, a) in fixture.enumerate() {}
		}
	}
}
