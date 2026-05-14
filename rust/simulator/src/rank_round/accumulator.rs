use dashmap::DashMap;
use dsv_writer::{Encoder, QuoteMode, ToDsv, ToDsvResult};
use std::collections::HashMap;
use std::io::Error;

const ROUND: usize = 68;
const OFFSET: usize = 4;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key {
	round: usize,
	rank: usize,
	tie: usize,
}

impl Key {
	pub fn new(round: usize, rank: usize, tie: usize) -> Self {
		Self { round, rank, tie }
	}

	pub fn round(&self) -> usize {
		self.round
	}

	pub fn rank(&self) -> usize {
		self.rank
	}

	pub fn tie(&self) -> usize {
		self.tie
	}
}

pub struct Accumulator {
	data: DashMap<Key, usize>,
	play_id: usize,
	player: usize,
	iteration: usize,
}

impl Accumulator {
	pub fn new(player: usize, play_id: usize, iteration: usize) -> Self {
		Self {
			data: DashMap::new(),
			play_id,
			player,
			iteration,
		}
	}

	pub fn input(&self, rounds: &[usize]) {
		let mut array = [0usize; ROUND];

		for &dta in rounds.iter() {
			array[dta - OFFSET] += 1;
		}

		for (rank, (round, &tie)) in array
			.iter()
			.enumerate()
			.map(|(round, tie)| (round + 4, tie))
			.filter(|&(_, &tie)| tie != 0)
			.enumerate()
		{
			let key = Key::new(round, rank + 1, tie);
			*self.data.entry(key).or_default() += 1;
		}
	}

	pub fn view(&self) -> &DashMap<Key, usize> {
		&self.data
	}

	pub fn write_header<T: Encoder>(writer: &mut T) -> ToDsvResult<(), Error> {
		writer.write_str_field("play_id", QuoteMode::AutoDetect)?;
		writer.write_str_field("iteration", QuoteMode::AutoDetect)?;
		writer.write_str_field("player", QuoteMode::AutoDetect)?;
		writer.write_str_field("round", QuoteMode::AutoDetect)?;
		writer.write_str_field("rank", QuoteMode::AutoDetect)?;
		writer.write_str_field("tie", QuoteMode::AutoDetect)?;
		writer.write_str_field("count", QuoteMode::AutoDetect)?;
		writer.end_of_record(false)?;
		Ok(())
	}
}

impl ToDsv<std::io::Error> for Accumulator {
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(), Error> {
		for x in self.data.iter() {
			writer.write_value_field(&self.play_id, QuoteMode::AutoDetect)?;
			writer.write_value_field(&self.iteration, QuoteMode::AutoDetect)?;
			writer.write_value_field(&self.player, QuoteMode::AutoDetect)?;

			writer.write_value_field(&x.key().round(), QuoteMode::AutoDetect)?;
			writer.write_value_field(&x.key().rank(), QuoteMode::AutoDetect)?;
			writer.write_value_field(&x.key().tie(), QuoteMode::AutoDetect)?;
			writer.write_value_field(&x.value(), QuoteMode::AutoDetect)?;
			writer.end_of_record(false)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use dsv_writer::{NewLine, RawWriter};
	use std::io::{Cursor, Write};
	#[test]
	fn to_dsv() {
		let mut background = Vec::<u8>::new();
		let cur = std::io::Cursor::new(&mut background);
		let mut writer = RawWriter::try_new(cur, '\t', NewLine::Lf).unwrap();

		let vec = vec![4, 4, 4, 12, 12, 34, 34, 34, 55, 55, 55];

		let mut fixture = Accumulator::new(10, 1, 1);
		fixture.input(&vec);

		let vec = vec![4, 4, 4, 4, 12, 13, 34, 34, 34, 55, 55, 55];
		fixture.input(&vec);

		let vec = vec![4, 4, 4, 4, 12, 13, 34, 34, 34, 55, 55, 55];
		fixture.input(&vec);

		Accumulator::write_header(&mut writer).unwrap();

		fixture.to_dsv(&mut writer).unwrap();
		let s = String::from_utf8(background.clone()).unwrap();
		println!("{}", s);
	}
}
