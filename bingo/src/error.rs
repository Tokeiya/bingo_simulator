pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Row {0} out of range")]
	RowOutOfRange(usize),
	#[error("Colon {0} out of range")]
	ColonOutOfRange(usize),
}
