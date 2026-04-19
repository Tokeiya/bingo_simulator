use bingo::Card;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use simulator::result_writer::{Element, ResultWriter};
use simulator::rng_helper::generate;
use std::cell;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

const ROUND: usize = 1_000_000;

thread_local! {
	static RNG:cell::RefCell<ChaCha20Rng> = cell::RefCell::new(generate());
}

fn main() {
	let mut file = fs::File::create("../data/case_five1.tsv").unwrap();
	_ = file.write(b"id\tcards\tround\tcount\n").unwrap();

	let first_hit: [AtomicUsize; 5] = [
		AtomicUsize::new(0),
		AtomicUsize::new(0),
		AtomicUsize::new(0),
		AtomicUsize::new(0),
		AtomicUsize::new(0),
	];

	let mut writer = ResultWriter::new();
	writer.start(file).unwrap();

	(0..ROUND).into_par_iter().for_each(|id| {
		RNG.with(|rng| {
			let mut cards = (0..5)
				.map(|_| Some(Card::new(&mut rng.borrow_mut(), true)))
				.collect::<Vec<_>>();

			let mut balls: [u8; 75] = std::array::from_fn(|i| i as u8);
			balls.shuffle(&mut rng.borrow_mut());

			let mut accum: [usize; 75] = [0; _];
			let mut cnt = 0usize;

			for (c, ball) in balls.into_iter().enumerate() {
				if cnt == 5 {
					break;
				}
				for card in cards.iter_mut() {
					if let Some(crd) = card {
						_ = crd.set(ball);
						if crd.remaining().view().contains(&0) {
							accum[c] += 1;
							*card = None;
							cnt += 1;
						}
					} else {
						continue;
					}
				}
			}

			let mut vec = Vec::<Element>::new();

			for (idx, cnt) in accum.iter().enumerate() {
				if *cnt == 0 {
					continue;
				}

				vec.push(Element {
					round: idx,
					count: *cnt,
				})
			}

			let a = accum.iter().find(|x| **x > 0).unwrap();
			first_hit[*a - 1].fetch_add(1, Ordering::Relaxed);

			writer.post(id, 5, vec);
			if id & 0x1F_FF == 0 {
				println!("Processed {} rounds", id);
			}
		})
	});

	writer.join().unwrap();

	for (i, a) in first_hit.iter().enumerate() {
		println!("{i}:{:?}", a);
	}
}
