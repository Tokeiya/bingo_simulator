use bingo::Card;
use dsv_writer::{Encoder, QuoteMode, RawWriter};
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_core::Rng;
use rayon::prelude::*;
use simulator::result_writer::ResultWriter;
use simulator::result_writer_error::*;
use std::sync::atomic::AtomicUsize;

const N: usize = 50;
const ROUND: usize = 100_000;
static SEED: AtomicUsize = AtomicUsize::new(0);

fn main() {
	let mut seed = [0u8; 32];
	rand::rng().fill_bytes(&mut seed);
	let file = std::fs::File::create("/mnt/wsl/data/output.tsv").unwrap();
	let mut writer = simulator::result_writer::ResultWriter::new();
	_ = writer.start(file).unwrap();

	let mut rands: Vec<ChaCha20Rng> = (0..20)
		.map(|x| {
			let mut rng = ChaCha20Rng::from_seed(seed);
			rng.set_stream(x);
			rng
		})
		.collect();

	_ = rands.par_iter_mut().for_each(|rng| {});

	// for i in 0..ROUND {
	// 	println!("Round: {}", i);
	// 	let ans = round(&mut rng);
	//
	// 	for (idx, cnt) in ans.iter().enumerate() {
	// 		writer.write_value_field(&i, QuoteMode::AutoDetect).unwrap();
	// 		writer
	// 			.write_value_field(&idx, QuoteMode::AutoDetect)
	// 			.unwrap();
	// 		writer
	// 			.write_value_field(&cnt, QuoteMode::AutoDetect)
	// 			.unwrap();
	// 		writer.end_of_record(false).unwrap();
	// 	}
	// }
}

fn round(rng: &mut ChaCha20Rng) -> [usize; 75] {
	let mut arr: Vec<_> = (0..N).map(|_| Option::Some(Card::new(rng, true))).collect();
	let mut accum: [usize; 75] = [0; _];

	let mut vec: Vec<u8> = (1..=75).collect();
	vec.shuffle(rng);

	for (cnt, ball) in vec.into_iter().enumerate() {
		for card in arr.iter_mut() {
			if let Some(c) = card {
				_ = c.set(ball);
				if c.remaining().view().contains(&0) {
					accum[cnt] += 1;
					*card = None;
				}
			} else {
				continue;
			}
		}
	}

	accum
}
