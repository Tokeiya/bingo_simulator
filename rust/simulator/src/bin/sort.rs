use dsv_reader::*;
use dsv_reader::*;
use dsv_writer::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const SCR: &str = "../data/sample1.tsv";
const OUT_DIR: &str = "/mnt/wsl/data/sort";

fn main() {
	verify();
}

fn verify() {
	let mut vec = Vec::new();

	for i in 1..=20 {
		let file = File::open(format!("{OUT_DIR}/{:03}.tsv", i * 5)).unwrap();
		let token = BufStream::<_, b'\t'>::try_new(file).unwrap();
		let mut reader = DsvReader::new(token);

		reader.fill(&mut vec).unwrap();
		vec.clear();

		let mut set = HashSet::<String>::new();

		for line in 0..usize::MAX {
			let cnt = reader.fill(&mut vec).unwrap();

			if cnt.is_none() || matches!(cnt,Some(c) if c==0) {
				break;
			}

			set.insert(vec[0].clone());

			vec.clear()
		}

		println!("{} cards: {}", i * 5, set.len());
	}
}

fn split() {
	let mut dict = HashMap::<usize, RawWriter<File>>::new();

	for i in 1..=20 {
		let file = File::create(format!("{OUT_DIR}/{:03}.tsv", i * 5)).unwrap();
		let mut writer = RawWriter::try_new(file, '\t').unwrap();

		writer
			.write_str_field("play_id", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("cards", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("round", QuoteMode::AutoDetect)
			.unwrap();
		writer
			.write_str_field("hit_count", QuoteMode::AutoDetect)
			.unwrap();

		writer.end_of_record(NewLineMode::Lf, false).unwrap();

		dict.insert(i * 5, writer);
	}

	let file = File::open(SCR).unwrap();
	let stream = BufStream::<_, b'\t'>::try_new(file).unwrap();
	let mut reader = DsvReader::new(stream);

	let mut vec = Vec::new();

	reader.fill(&mut vec).unwrap();
	vec.clear();

	for i in 0..usize::MAX {
		let cnt = reader.fill(&mut vec).unwrap();

		if cnt.is_none() || matches!(cnt,Some(c) if c==0) {
			break;
		}

		let cards = vec[1].parse::<usize>().unwrap();

		write(vec.as_slice(), dict.get_mut(&cards).unwrap());

		if i & 0xFF_FF == 0 {
			println!("{:04}:{:?}", i, &vec);
		}

		vec.clear();
	}
}

fn write(scr: &[String], writer: &mut RawWriter<File>) {
	for s in scr {
		writer.write_str_field(s, QuoteMode::AutoDetect).unwrap();
	}

	writer.end_of_record(NewLineMode::Lf, false).unwrap();
}
