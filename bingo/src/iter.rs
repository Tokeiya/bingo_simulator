use crate::error;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	Row,
	Column,
	MainDiagonal,
	AntiDiagonal,
}

pub struct Iter {
	pivot: u8,
	offset: u8,
	direction: Direction,
}

impl Iter {
	pub fn try_new(linear: u8, direction: Direction) -> error::Result<Self> {
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
