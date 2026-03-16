use super::error::{Error, Result};

pub const ROW_SIZE: usize = 5;
pub const COLUMN_SIZE: usize = 5;
pub const LINEAR_SIZE: usize = ROW_SIZE * COLUMN_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct RowCol {
	row: usize,
	col: usize,
}

fn to_coordinate(linear: usize) -> (usize, usize) {
	let row = linear % ROW_SIZE;
	let col = linear / ROW_SIZE;

	(row, col)
}

fn to_linear(row: usize, col: usize) -> usize {
	row + col * COLUMN_SIZE
}

impl RowCol {
	pub fn try_from_coordinate(row: usize, col: usize) -> Result<Self> {
		if row >= ROW_SIZE {
			return Err(Error::RowOutOfRange(row));
		}
		if col >= COLUMN_SIZE {
			return Err(Error::ColumnOutOfRange(col));
		}
		Ok(RowCol { row, col })
	}

	pub fn try_from_linear(value: usize) -> Result<Self> {
		if value >= LINEAR_SIZE {
			return Err(Error::LinearOutOfRange(value));
		}

		let (row, col) = to_coordinate(value);
		Ok(RowCol { row, col })
	}

	pub fn row(&self) -> usize {
		self.row
	}

	pub fn col(&self) -> usize {
		self.col
	}

	pub fn linear(&self) -> usize {
		to_linear(self.row, self.col)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::error::Error;
	
	fn to_coordinate(linear: usize) -> (usize, usize) {
		let col = linear / ROW_SIZE;
		let row = linear % ROW_SIZE;
		(row, col)
	}

	fn to_linear(row: usize, col: usize) -> usize {
		row + col * 5
	}

	#[test]
	fn try_from_coordinate() {
		for r in 0..ROW_SIZE {
			for c in 0..COLUMN_SIZE {
				let fixture = RowCol::try_from_coordinate(r, c).unwrap();
				assert_eq!(fixture.row, r);
				assert_eq!(fixture.col, c);
			}
		}

		assert!(
			matches!(RowCol::try_from_coordinate(0,COLUMN_SIZE),Err(Error::ColumnOutOfRange(c)) if c==COLUMN_SIZE)
		);

		assert!(
			matches!(RowCol::try_from_coordinate(ROW_SIZE,0),Err(Error::RowOutOfRange(r)) if r==ROW_SIZE)
		);
	}

	#[test]
	fn try_from_linear() {
		for i in 0..COLUMN_SIZE {
			let fixture = RowCol::try_from_linear(i).unwrap();
			let (r, c) = to_coordinate(i);
			assert_eq!(fixture.row, r);
			assert_eq!(fixture.col, c);
		}

		assert!(
			matches!(RowCol::try_from_linear(COLUMN_SIZE*ROW_SIZE),Err(Error::LinearOutOfRange(l)) if l==COLUMN_SIZE*ROW_SIZE)
		);
	}

	#[test]
	fn row() {
		for r in 0..ROW_SIZE {
			for c in 0..COLUMN_SIZE {
				let fixture = RowCol::try_from_coordinate(r, c).unwrap();
				assert_eq!(fixture.row(), r);
			}
		}
	}

	#[test]
	fn col() {
		for r in 0..ROW_SIZE {
			for c in 0..COLUMN_SIZE {
				let fixture = RowCol::try_from_coordinate(r, c).unwrap();
				assert_eq!(fixture.col(), c);
			}
		}
	}

	#[test]
	fn linear() {
		for r in 0..ROW_SIZE {
			for c in 0..COLUMN_SIZE {
				let fixture = RowCol::try_from_coordinate(r, c).unwrap();
				assert_eq!(fixture.linear(), to_linear(r, c));
			}
		}
	}

	#[test]
	fn fool_proof() {
		let fixture = RowCol::try_from_coordinate(2, 2).unwrap();
		assert_eq!(fixture.linear(), 12);

		let fixture = RowCol::try_from_linear(12).unwrap();
		assert_eq!(fixture.row(), 2);
		assert_eq!(fixture.col(), 2);
	}
}
