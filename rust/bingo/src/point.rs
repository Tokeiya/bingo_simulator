use super::error::*;

pub const EDGE_SIZE: u8 = 5;

pub struct Point(u8);

impl Point {
	pub fn try_from_linear(linear: u8) -> Result<Self> {
		if linear >= 25 {
			Err(Error::LinearOutOfRange(linear as usize))
		} else {
			Ok(Point(linear))
		}
	}
	pub fn try_from_grid(row: u8, col: u8) -> Result<Self> {
		if row >= EDGE_SIZE {
			Err(Error::RowOutOfRange(row as usize))
		} else if col >= EDGE_SIZE {
			Err(Error::ColumnOutOfRange(col as usize))
		} else {
			Ok(Point(row + col * EDGE_SIZE))
		}
	}
	pub fn row(&self) -> u8 {
		self.0 % EDGE_SIZE
	}
	pub fn column(&self) -> u8 {
		self.0 / EDGE_SIZE
	}
	pub fn linear(&self) -> usize {
		self.0 as usize
	}
	pub fn lies_on_main_diagonal(&self) -> bool {
		self.0 % 6 == 0
	}
	pub fn lies_on_anti_diagonal(&self) -> bool {
		if self.0 == 0 || self.0 == 24 {
			false
		} else {
			self.0 % 4 == 0
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	
	fn to_linear(row: u8, col: u8) -> u8 {
		col * 5 + row
	}

	fn to_grid(linear: u8) -> (u8, u8) {
		(linear % 5, linear / 5)
	}

	#[test]
	fn try_from_linear() {
		for i in 0..25 {
			let fixture = Point::try_from_linear(i).unwrap();
			assert_eq!(fixture.0, i)
		}

		assert!(matches!(Point::try_from_linear(25), Err(Error::LinearOutOfRange(x)) if x==25));
	}

	#[test]
	fn try_from_grid() {
		for r in 0..5 {
			for c in 0..5 {
				let fixture = Point::try_from_grid(r, c).unwrap();
				assert_eq!(fixture.0, to_linear(r, c))
			}
		}

		assert!(matches!(Point::try_from_grid(5,0),Err(Error::RowOutOfRange(x)) if x==5));
		assert!(matches!(Point::try_from_grid(0,5),Err(Error::ColumnOutOfRange(x)) if x==5));
	}

	#[test]
	fn row() {
		for i in 0..25 {
			let fixture = Point(i);
			let (e, _) = to_grid(i);
			assert_eq!(fixture.row(), e);
		}
	}

	#[test]
	fn column() {
		for i in 0..25 {
			let fixture = Point(i);
			let (_, e) = to_grid(i);
			assert_eq!(fixture.column(), e);
		}
	}

	#[test]
	fn linear() {
		for i in 0..25 {
			let fixture = Point(i);
			assert_eq!(fixture.linear(), i as usize);
		}
	}

	#[test]
	fn lies_on_main_diagonal() {
		for (idx, fix) in (0..25).map(|x| Point(x)).enumerate() {
			assert_eq!(fix.lies_on_main_diagonal(), idx % 6 == 0);
		}
	}

	#[test]
	fn lies_on_anti_diagonal() {
		const EXPECTED: [usize; 5] = [4, 8, 12, 16, 20];

		for (idx, fix) in (0..25).map(|x| Point(x)).enumerate() {
			assert_eq!(fix.lies_on_anti_diagonal(), EXPECTED.contains(&idx));
		}
	}
}
