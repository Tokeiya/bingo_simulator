use super::error::*;
use super::point::Point;

pub(super) const ROW_IDX: [usize; 5] = [0, 1, 2, 3, 4];
pub(super) const COL_IDX: [usize; 5] = [5, 6, 7, 8, 9];
pub(super) const MAIN_DIAGONAL: usize = 10;
pub(super) const ANTI_DIAGONAL: usize = 11;
pub struct Remaining([u8; 12]);

impl Remaining {
	pub fn new() -> Self {
		Self([5; _])
	}
	pub fn row(&self, idx: usize) -> Result<usize> {
		if idx >= 5 {
			Err(Error::RowOutOfRange(idx))
		} else {
			Ok(self.0[ROW_IDX[idx]] as usize)
		}
	}
	pub fn column(&self, idx: usize) -> Result<usize> {
		if idx >= 5 {
			Err(Error::ColumnOutOfRange(idx))
		} else {
			Ok(self.0[COL_IDX[idx]] as usize)
		}
	}
	pub fn main_diagonal(&self) -> usize {
		self.0[MAIN_DIAGONAL] as usize
	}
	pub fn anti_diagonal(&self) -> usize {
		self.0[ANTI_DIAGONAL] as usize
	}
	fn decrement(&mut self, point: &Point) -> Result<()> {
		let row = ROW_IDX[point.row() as usize];
		let col = COL_IDX[point.column() as usize];
		let lies_main = point.lies_on_main_diagonal();
		let lies_anti = point.lies_on_anti_diagonal();

		if self.0[row] == 0 {
			Err(Error::RemainingIsZero)
		} else if self.0[col] == 0 {
			Err(Error::RemainingIsZero)
		} else if lies_main && self.0[MAIN_DIAGONAL] == 0 {
			Err(Error::RemainingIsZero)
		} else if lies_anti && self.0[ANTI_DIAGONAL] == 0 {
			Err(Error::RemainingIsZero)
		} else {
			self.0[row] -= 1;
			self.0[col] -= 1;
			if lies_main {
				self.0[MAIN_DIAGONAL] -= 1;
			}
			if lies_anti {
				self.0[ANTI_DIAGONAL] -= 1;
			}

			Ok(())
		}
	}
	fn view(&self) -> &[u8] {
		&self.0
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
		let mut cnt = 5;

		for i in ROW_IDX.iter() {
			fixture.0[*i] = cnt;
			cnt -= 1;
		}

		cnt = 5;

		for i in 0..5 {
			dbg!(i);
			dbg!(fixture.row(i).unwrap());
			assert_eq!(fixture.row(i).unwrap(), cnt.into());
			cnt -= 1;
		}

		assert!(matches!(fixture.row(5),Err(Error::RowOutOfRange(x)) if x==5));
	}

	#[test]
	fn col() {
		let fixture = Remaining(std::array::from_fn(|x| x as u8));

		for i in 0..5 {
			assert_eq!(fixture.column(i).unwrap(), i + 5);
		}

		assert!(matches!(fixture.column(5), Err(Error::ColumnOutOfRange(x)) if x==5))
	}

	#[test]
	fn main_diagonal() {
		let mut fixture = Remaining([1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 5, 4]);

		assert_eq!(fixture.main_diagonal(), 5);
	}

	#[test]
	fn decrement() {
		let mut fixture = Remaining([5; 12]);
		let p = Point::try_from_linear(0).unwrap();

		assert_eq!(fixture.0[0], 5);

		for i in (0..5).rev() {
			fixture.decrement(&p).unwrap();
			assert_eq!(fixture.0[ROW_IDX[0]], i);
		}

		for (idx, cnt) in fixture.0.iter().enumerate() {
			println!("[{idx}]={cnt}");
		}

		assert!(matches!(fixture.decrement(&p), Err(Error::RemainingIsZero)));
	}

	#[test]
	fn view() {
		let fixture = Remaining(std::array::from_fn(|x| x as u8));

		assert_eq!(&fixture.0, fixture.view());
	}
}
