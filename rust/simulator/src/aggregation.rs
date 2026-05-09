use dsv_writer::*;

pub const ROUND_OFFSET: usize = 4;
const ROUND_SIZE: usize = 68;
pub struct Table<const N: usize> {
	data: [[usize; ROUND_SIZE]; N],
}

impl<const N: usize> Default for Table<N> {
	fn default() -> Self {
		Self {
			data: [[0; ROUND_SIZE]; N],
		}
	}
}

impl<const N: usize> Table<N> {
	pub fn set(&mut self, data: &[usize]) {
		for rank in 0..data.len() {
			let round = data[rank];
			self.data[rank][round - ROUND_OFFSET] += 1;
		}
	}

	pub fn view(&self) -> &[[usize; ROUND_SIZE]] {
		&self.data
	}
}

impl<const N: usize> ToDsv<std::io::Error> for Table<N> {
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(), std::io::Error> {
		writer.write_str_field("rank", QuoteMode::AutoDetect)?;

		for idx in ROUND_OFFSET..=ROUND_OFFSET + ROUND_SIZE {
			writer.write_value_field(&idx, QuoteMode::AutoDetect)?;
		}

		writer.end_of_record(false)?;

		for rank in 0..self.data.len() {
			writer.write_value_field(&rank, QuoteMode::AutoDetect)?;

			for value in self.data[rank].iter() {
				writer.write_value_field(&value, QuoteMode::AutoDetect)?;
			}

			writer.end_of_record(false)?;
		}

		Ok(())
	}
}
