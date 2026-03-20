use super::error::*;
use super::point::Point;

pub(super) const ROW_IDX: [usize; 5] = [0, 1, 2, 3, 4];
pub(super) const COL_IDX: [usize; 5] = [5, 6, 7, 8, 9];
pub(super) const MAIN_DIAGONAL: usize = 10;
pub(super) const ANTI_DIAGONAL: usize = 11;
pub struct Remaining([u8; 12]);

impl Remaining {
	pub fn new() -> Self {
		todo!()
	}
	pub fn row(&self, idx: usize) -> Result<usize> {
		todo!()
	}
	pub fn column(&self, idx: usize) -> Result<usize> {
		todo!()
	}
	pub fn main_diagonal(&self) -> usize {
		todo!()
	}
	pub fn anti_diagonal(&self) -> usize {
		todo!()
	}
	fn decrement(&mut self, point: &Point) -> Result<bool> {
		todo!()
	}
	fn view(&self) -> &[u8] {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::error::*;
	
	#[test]
	fn new() {
		let fixture = Remaining::new();
		assert!(fixture.0.iter().all(|x| x == &5))
	}

	#[test]
	fn row() {
		let mut fixture = Remaining::new();
		let mut cnt = 0;

		for i in ROW_IDX.iter() {
			fixture.0[*i] = cnt;
			cnt -= 1;
		}

		for i in 0..5 {
			assert_eq!(fixture.row(i).unwrap(), i)
		}

		assert!(matches!(fixture.row(5),Err(Error::RowOutOfRange(x)) if x==5));
	}

	#[test]
	fn main_diagonal() {
		let mut fixture = Remaining([1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 5, 4]);

		assert_eq!(fixture.main_diagonal(), 5);
	}

	#[test]
	fn decrement() {
		let mut fixture = Remaining([5; 12]);

		for i in (0..5).map(|x| x * 5) {}
	}
}
