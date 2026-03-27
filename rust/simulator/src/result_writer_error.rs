use dsv_writer::Error as DsvWriterError;
use std::any::Any;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid number of elements expected {0} but got {1}")]
	InvalidNumberOfElements(usize, usize),
	#[error(transparent)]
	IOError(#[from] std::io::Error),
	#[error("Handle already closed")]
	HandleAlreadyClosed,
	#[error("BoxResult")]
	BoxResult(Box<dyn Any + Send + 'static>),
}

impl From<dsv_writer::Error> for Error {
	fn from(value: DsvWriterError) -> Self {
		match value {
			DsvWriterError::InvalidNumberOfElements(e, a) => Self::InvalidNumberOfElements(e, a),
			DsvWriterError::IOError(ioe) => Self::IOError(ioe),
		}
	}
}

impl From<Box<dyn Any + Send + 'static>> for Error {
	fn from(value: Box<dyn Any + Send + 'static>) -> Self {
		Self::BoxResult(value)
	}
}

impl Error {
	pub fn handle_already_closed() -> Self {
		Self::HandleAlreadyClosed
	}
}
