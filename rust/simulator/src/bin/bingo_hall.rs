use bingo::Card;
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_core::Rng;
use rayon::prelude::*;
use simulator::result_writer::ResultWriter;
use std::cell;
use std::io::Write;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

const ROUND: usize = 5;

static STREAM_ID: AtomicU64 = AtomicU64::new(0);

static SEED: LazyLock<[u8; 32]> = LazyLock::new(|| {
	let mut arr = [0; 32];
	rand::rng().fill_bytes(&mut arr);
	arr
});

fn generate() -> ChaCha20Rng {
	let mut rng = ChaCha20Rng::from_seed(*SEED);
	let id = STREAM_ID.fetch_add(1, Ordering::Relaxed);
	rng.set_stream(id);
	rng
}

thread_local! {
	static RNG:cell::RefCell<ChaCha20Rng> = cell::RefCell::new(generate());
}

fn main() {
	let mut file = std::fs::File::create("/mnt/wsl/data/sample.tsv").unwrap();
	_ = file.write(b"id\tcards\tround\tcount").unwrap();

	let mut writer = ResultWriter::new();
	writer.start(file).unwrap();

	let id_seed = AtomicUsize::new(0);

	for n in (1..=20).map(|i| i * 5) {
		println!("Processing {} cards", n);
		(0..ROUND).par_bridge().for_each(|_| {
			let id = id_seed.fetch_add(1, Ordering::Relaxed);

			RNG.with(|rng| {
				let mut cards = (0..n)
					.map(|_| Some(Card::new(&mut rng.borrow_mut(), true)))
					.collect::<Vec<_>>();

				let mut balls: [u8; 75] = std::array::from_fn(|i| i as u8);
				balls.shuffle(&mut rng.borrow_mut());

				let mut accum: [usize; 75] = [0; _];

				for (c, ball) in balls.iter().enumerate() {
					for card in cards.iter_mut() {
						if let Some(crd) = card {
							_ = crd.set(*ball);
							if crd.remaining().view().contains(&0) {
								accum[c] += 1;
								*card = None;
							}
						} else {
							continue;
						}
					}
				}

				writer.post(id, n, accum);
			})
		})
	}

	writer.join().unwrap();
}
