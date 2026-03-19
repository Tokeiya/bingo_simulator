use super::error::*;
use super::iter::Iter;
pub struct Point(u8);

impl Point {
	pub fn try_from_linear(value: u8) -> Result<Self> {
		todo!()
	}

	pub fn try_from_grid(row: u8, col: u8) -> Result<Self> {
		todo!()
	}

	pub fn row(&self) -> u8 {
		todo!()
	}

	pub fn col(&self) -> u8 {
		todo!()
	}

	pub fn linear(&self) -> usize {
		todo!()
	}

	pub fn top(&self) -> Self {
		todo!()
	}

	pub fn left(&self) -> Self {
		todo!()
	}

	pub fn row_idx_iter(&self) -> Iter {
		todo!()
	}

	pub fn col_idx_iter(&self) -> Iter {
		todo!()
	}
}
