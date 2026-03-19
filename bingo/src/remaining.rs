use super::error::*;

pub(super) const ROW_IDX: [usize; 5] = [0, 1, 2, 3, 4];
pub(super) const COL_IDX: [usize; 5] = [5, 6, 7, 8, 9];
pub(super) const MAIN_DIAGONAL: usize = 10;
pub(super) const ANTI_DIAGONAL: usize = 11;

pub(super) trait RemainingEdit {
	fn inclement_row(&mut self) -> Result<usize>;
	fn inclement_col(&mut self) -> Result<usize>;
	fn inclement_main_diagonal(&mut self) -> Result<usize>;
	fn inclement_anti_diagonal(&mut self) -> Result<usize>;
}

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

	fn decrement_row(&mut self, index: usize) -> Result<usize> {
		todo!()
	}
	fn decrement_col(&mut self, index: usize) -> Result<usize> {
		todo!()
	}
	fn decrement_main_diagonal(&mut self) -> Result<usize> {
		todo!()
	}
	fn decrement_anti_diagonal(&mut self) -> Result<usize> {
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
			cnt += 1;
		}

		for i in 0..5 {
			assert_eq!(fixture.row(i).unwrap(), i)
		}

		assert!(matches!(fixture.row(5),Err(Error::RowOutOfRange(x)) if x==5));
	}

	#[test]
	fn decrement_row() {
		let mut fixture = Remaining::new();

		assert!(matches!(fixture.decrement_row(5),Err(Error::RowOutOfRange(x)) if x==5));

		for i in 1..=5 {
			for r in 0..5 {
				let act = fixture.decrement_row(r).unwrap();
				assert_eq!(act, 5 - i);
				assert_eq!(fixture.0[ROW_IDX[r]], (5 - i) as u8);
			}
		}

		for i in 0..5 {
			assert!(matches!(
				fixture.decrement_row(i),
				Err(Error::RemainingIsZero)
			))
		}
	}

	#[test]
	fn decrement_col() {
		let mut fixture = Remaining::new();

		assert!(matches!(fixture.decrement_col(5),Err(Error::ColumnOutOfRange(x)) if x==5));

		for i in 1..=5 {
			for c in 0..5 {
				let act = fixture.decrement_col(c).unwrap();
				assert_eq!(act, 5 - i);
				assert_eq!(fixture.0[COL_IDX[c]], (5 - 1) as u8);
			}
		}

		for i in 0..5 {
			assert!(matches!(
				fixture.decrement_col(i),
				Err(Error::RemainingIsZero)
			))
		}
	}

	#[test]
	fn main_diagonal() {
		let mut fixture = Remaining([1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 5, 4]);

		assert_eq!(fixture.main_diagonal(), 5);
	}

	#[test]
	fn decrement_main_diagonal() {
		todo!()
	}
}
