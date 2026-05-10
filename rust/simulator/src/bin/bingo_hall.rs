use bingo::Card;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use simulator::result_writer::{Element, ResultWriter};
use simulator::rng_helper::generate;
use std::cell;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

const ROUND: usize = 1_000_000;

thread_local! {
	static RNG:cell::RefCell<ChaCha20Rng> = cell::RefCell::new(generate());
}

//noinspection DuplicatedCode
fn main() {
	let mut file = std::fs::File::create("../data/sample1.tsv").unwrap();
	//	let mut file = std::fs::File::create("/dev/null").unwrap();

	_ = file.write(b"play_id\tcards\tround\thit_count\n").unwrap();

	let mut writer = ResultWriter::new();
	writer.start(file).unwrap();

	let id_seed = AtomicUsize::new(0);

	for n in (1..=20).map(|i| i * 5) {
		println!("Processing {} cards", n);
		(0..ROUND).into_par_iter().for_each(|_| {
			let id = id_seed.fetch_add(1, Ordering::Relaxed);

			RNG.with(|rng| {
				let mut cards = (0..n)
					.map(|_| Some(Card::new(&mut rng.borrow_mut(), true)))
					.collect::<Vec<_>>();

				let mut balls: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
				balls.shuffle(&mut rng.borrow_mut());

				let mut accum: [usize; 76] = [0; _];
				let mut card_count = 0usize;

				'outer: for (c, ball) in balls.iter().enumerate() {
					for card in cards.iter_mut() {
						if let Some(crd) = card {
							_ = crd.set(*ball);
							if crd.remaining().hit_count() != 0 {
								accum[c + 1] += 1;
								*card = None;
								card_count += 1;
								if card_count == n {
									break 'outer;
								}
							}
						} else {
							continue;
						}
					}
				}

				let mut vec = Vec::<Element>::new();

				for (idx, &cnt) in accum.iter().enumerate() {
					if cnt == 0 {
						continue;
					}

					vec.push(Element {
						round: idx,
						count: cnt,
					})
				}

				writer.post(id, n, vec);
			})
		})
	}

	writer.join().unwrap();
}
