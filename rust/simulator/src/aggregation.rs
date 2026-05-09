use dsv_writer::*;

pub const ROUND_OFFSET: usize = 4;
const ROUND_SIZE: usize = 68;
const RANK_SIZE: usize = ROUND_SIZE;
pub struct Table {
	data: [[usize; ROUND_SIZE]; RANK_SIZE],
}

impl Default for Table {
	fn default() -> Self {
		Self {
			data: [[0; RANK_SIZE]; ROUND_SIZE],
		}
	}
}

impl Table {
	pub fn set(&mut self, rounds: &[usize]) {
		let mut recent = 0usize;
		let mut rank = 0usize;

		for &round in rounds {
			if recent != round {
				recent = round;
				rank += 1;
			}

			self.data[round - ROUND_OFFSET][rank] += 1;
		}
	}

	pub fn view(&self) -> &[[usize; RANK_SIZE]; ROUND_SIZE] {
		&self.data
	}
}

impl ToDsv<std::io::Error> for Table {
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(), std::io::Error> {
		writer.write_str_field("round", QuoteMode::AutoDetect)?;
		writer.write_str_field("rank", QuoteMode::AutoDetect)?;
		writer.write_str_field("count", QuoteMode::AutoDetect)?;
		writer.end_of_record(false)?;

		for round in 0..ROUND_SIZE {
			for (rank, &count) in self.data[round].iter().enumerate() {
				writer.write_value_field(&(round + ROUND_OFFSET), QuoteMode::AutoDetect)?;
				writer.write_value_field(&rank, QuoteMode::AutoDetect)?;
				writer.write_value_field(&count, QuoteMode::AutoDetect)?;
				writer.end_of_record(false)?;
			}
		}

		Ok(())
	}
}
