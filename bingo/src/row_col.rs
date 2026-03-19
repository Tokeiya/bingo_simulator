use super::error::{Error, Result};

pub const ROW_SIZE: usize = 5;
pub const COLUMN_SIZE: usize = 5;
pub const LINEAR_SIZE: usize = ROW_SIZE * COLUMN_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct RowCol {
	row: usize,
	col: usize,
}

#[inline(always)]
pub fn try_to_coordinate(linear: usize) -> Result<RowCol> {
	if linear >= LINEAR_SIZE {
		return Err(Error::LinearOutOfRange(linear));
	}

	let row = linear % ROW_SIZE;
	let col = linear / ROW_SIZE;

	Ok(RowCol { row, col })
}

#[inline(always)]
pub fn try_to_linear(row: usize, col: usize) -> Result<usize> {
	if row >= ROW_SIZE {
		return Err(Error::RowOutOfRange(row));
	}
	if col >= COLUMN_SIZE {
		return Err(Error::ColumnOutOfRange(col));
	}
	Ok(row + col * COLUMN_SIZE)
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
		try_to_coordinate(value)
	}

	pub fn row(&self) -> usize {
		self.row
	}

	pub fn col(&self) -> usize {
		self.col
	}

	pub fn linear(&self) -> usize {
		try_to_linear(self.row, self.col).unwrap()
	}

	pub fn left_edge(&self) -> Self {
		Self::try_from_coordinate(self.row, 0).unwrap()
	}

	pub fn top_edge(&self) -> Self {
		Self::try_from_coordinate(0, self.col).unwrap()
	}

	pub fn offset(&self, row: i8, col: i8) -> Result<Self> {
		let r = row + self.row as i8;
		if r < 0 || r >= ROW_SIZE as i8 {
			return Err(Error::RowOutOfRange(r as usize));
		}
		let c = self.col as i8 + col;
		if c < 0 || c >= COLUMN_SIZE as i8 {
			return Err(Error::ColumnOutOfRange(c as usize));
		}
		Self::try_from_coordinate(r as usize, c as usize)
	}

	pub fn diag_edge(&self) -> Option<Self> {
		if self.linear() == 12 {
			unreachable!()
		}

		if (self.row == self.col) {
			Some(Self::try_from_coordinate(0, 0).unwrap())
		} else if (self.row + self.col == 4) {
			Some(Self::try_from_coordinate(0, 4).unwrap())
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::error::Error;
	
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
			assert_eq!(fixture.row, i % 5);
			assert_eq!(fixture.col, i / 5);
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
				assert_eq!(fixture.linear(), c * COLUMN_SIZE + r);
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

	#[test]
	fn try_to_linear_test() {
		for r in 0..ROW_SIZE {
			for c in 0..COLUMN_SIZE {
				assert_eq!(try_to_linear(r, c).unwrap(), c * COLUMN_SIZE + r);
			}
		}

		assert!(matches!(try_to_linear(ROW_SIZE, 0),Err(Error::RowOutOfRange(r)) if r==ROW_SIZE));
		assert!(
			matches!(try_to_linear(0, COLUMN_SIZE),Err(Error::ColumnOutOfRange(c)) if c==COLUMN_SIZE)
		);
	}

	#[test]
	fn try_to_coordinate_test() {
		for i in 0..25 {
			let fixture = try_to_coordinate(i).unwrap();

			assert_eq!(fixture.row, i % 5);
			assert_eq!(fixture.col, i / 5);
		}

		assert!(matches!(try_to_coordinate(25),Err(Error::LinearOutOfRange(l)) if l==25));
	}

	#[test]
	fn top_edge() {
		for i in 0..5 {
			let fixture = RowCol::try_from_coordinate(2, i);
			let act = fixture.unwrap().top_edge();
			assert_eq!(act.linear(), i * 5);
		}
	}

	#[test]
	fn left_edge() {
		for i in 0..5 {
			let fixture = RowCol::try_from_coordinate(i, 2).unwrap();
			let act = fixture.left_edge();
			assert_eq!(act.linear(), i);
		}
	}

	#[test]
	fn diag_edge() {
		for i in [0, 6, 18, 24] {
			let fixture = RowCol::try_from_linear(i).unwrap();
			let act = fixture.diag_edge().unwrap();
			assert_eq!(act.linear(), 0);
		}

		let fixture = RowCol::try_from_linear(2).unwrap();
		assert!(fixture.diag_edge().is_none());

		for i in [4, 8, 16, 20] {
			let fixture = RowCol::try_from_linear(i).unwrap();
			let act = fixture.diag_edge().unwrap();
			assert_eq!(act.linear(), 20);
		}

		let fixture = RowCol::try_from_linear(3).unwrap();
		assert!(fixture.diag_edge().is_none());
	}

	#[should_panic]
	#[test]
	fn unreachable_diag_edge() {
		let fixture = RowCol::try_from_linear(12);
		_ = fixture.unwrap().diag_edge()
	}

	#[test]
	fn offset() {
		let fixture = RowCol::try_from_linear(12).unwrap();
		let act = fixture.offset(1, 1).unwrap();
		assert_eq!(act.linear(), 18);

		assert_eq!(act.offset(-1, -1).unwrap().linear(), 12)
	}
}
