pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Row {0} out of range")]
	RowOutOfRange(usize),
	#[error("Column {0} out of range")]
	ColumnOutOfRange(usize),
	#[error("Cell (R:{0},C:{1}) already upped")]
	AlreadyUpped(usize, usize),
	#[error("Invalid cell value")]
	InvalidCellValue(u8),
}
