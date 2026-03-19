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
	#[error("Linear index {0} out of range")]
	LinearOutOfRange(usize),
	#[error("Linear index {0} Main diagonal cannot be defined")]
	MainDiagonalCantDefined(usize),
	#[error("Linear index {0} Anti diagonal cannot be defined")]
	AntiDiagonalCantDefined(usize),
}
