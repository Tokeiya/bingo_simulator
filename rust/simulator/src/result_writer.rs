use super::result_writer_error::{Error as ResultWriterError, Result as ResultWriterResult};
use crossbeam_channel::{Receiver, Sender};
use dsv_writer::{NewLineMode, Result as DsvWriterResult};
use dsv_writer::{Encoder, QuoteMode, RawWriter};
use std::fs;
use std::thread::JoinHandle;
pub struct Datum {
	result: [usize; 75],
	num: usize,
	id: usize,
}

pub struct ResultWriter {
	tx: Sender<Option<Datum>>,
	rx: Receiver<Option<Datum>>,
	handle: Option<JoinHandle<DsvWriterResult<()>>>,
}

impl Default for ResultWriter {
	fn default() -> Self {
		Self::new()
	}
}

impl ResultWriter {
	pub fn new() -> Self {
		let (tx, rx) = crossbeam_channel::bounded::<Option<Datum>>(10);

		Self {
			tx,
			rx,
			handle: None,
		}
	}

	pub fn start(&mut self, file: fs::File) -> Result<(), common_errors::invalid_argument::Error> {
		let writer = dsv_writer::RawWriter::try_new(file, '\t')?;
		let r = self.rx.clone();
		let handle = std::thread::spawn(move || Self::thread_proc(r, writer));

		self.handle = Some(handle);

		Ok(())
	}

	pub fn post(&self, id: usize, num: usize, data: [usize; 75]) {
		let datum = Datum {
			id,
			num,
			result: data,
		};
		self.tx.send(Some(datum)).unwrap();
	}

	pub fn join(&mut self) -> ResultWriterResult<()> {
		let handle = match self.handle.take() {
			None => return Err(ResultWriterError::handle_already_closed()),
			Some(h) => h,
		};

		self.tx.send(None).unwrap();

		let result = match handle.join() {
			Ok(v) => v,
			Err(err) => return Err(ResultWriterError::from(err)),
		};

		Ok(result?)
	}

	fn thread_proc(
		rx: Receiver<Option<Datum>>,
		mut writer: RawWriter<fs::File>,
	) -> DsvWriterResult<()> {
		loop {
			let datum = rx.recv().unwrap();

			let datum = if let Some(d) = datum {
				d
			} else {
				break;
			};

			for (idx, value) in datum.result.iter().enumerate() {
				writer.write_value_field(&datum.id, QuoteMode::AutoDetect)?;
				writer.write_value_field(&datum.num, QuoteMode::AutoDetect)?;
				writer.write_value_field(&idx, QuoteMode::AutoDetect)?;
				writer.write_value_field(&value, QuoteMode::AutoDetect)?;
				writer.end_of_record(NewLineMode::Lf,false)?;
			}
		}

		Ok(())
	}
}
