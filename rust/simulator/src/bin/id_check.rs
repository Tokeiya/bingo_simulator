use dsv_reader::DsvReader;
use dsv_reader::{BufStream, DsvRead};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

fn main() {
	let file = File::open("../data/sample1.tsv").unwrap();
	let file = BufReader::new(file);
	let stream = BufStream::<_, b'\t'>::try_new(file).unwrap();
	let mut reader = DsvReader::new(stream);

	let mut vec = Vec::new();

	_ = reader.fill(&mut vec).unwrap();
	vec.clear();

	let mut set = HashSet::<usize>::new();

	for i in 0..usize::MAX {
		let cnt = reader.fill(&mut vec).unwrap();

		if cnt.is_none() || matches!(cnt,Some(c) if c==0) {
			break;
		}

		if vec[1] == "5" {
			let id = vec[0].parse::<usize>().unwrap();
			set.insert(id);

			if i & 0xff_ff == 0 {
				println!("{}: id:{id}", i);
			}
		}

		vec.clear();
	}

	println!("len {:?}", set.len());
}
