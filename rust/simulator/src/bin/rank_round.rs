use bingo::Card;
use clap::Parser;
use dsv_writer::{NewLine, ToDsv};
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use simulator::aggregation::Table;
use simulator::rng_helper::generate;
use std::cell;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
	#[arg(short = 'i', long)]
	iteration: usize,
	#[arg(short = 'p', long)]
	player: usize,
	#[arg(short, long, default_value = "../data/")]
	path: String,
}

impl Args {
	pub fn create_path(&self, channel: usize) -> PathBuf {
		Path::new(&self.path).join(format!("{:00}_{channel:00}.tsv", self.player))
	}
}

thread_local! {
	static RNG:cell::RefCell<ChaCha20Rng> = cell::RefCell::new(generate());
	static ACCUM:cell::RefCell<Table> = cell::RefCell::new(Table::default());
}

fn main() {
	let args = Args::parse();

	(0..20usize).into_par_iter().for_each(|channel| {
		for i in 0..args.iteration {
			play(args.player);

			if i & 0xffff == 0 {
				println!("{channel} {i} / {} completed", args.iteration)
			}
		}

		let mut writer = dsv_writer::RawWriter::try_new(
			std::fs::File::create(args.create_path(channel)).unwrap(),
			'\t',
			NewLine::Lf,
		)
		.unwrap();

		ACCUM.with(|a| a.borrow().to_dsv(&mut writer).unwrap());
	});
}

fn initialize(n: usize) -> ([u8; 75], Vec<Option<Card>>) {
	let mut balls = std::array::from_fn::<_, 75, _>(|i| (i + 1) as u8);
	let mut vec: Vec<Option<Card>> = Vec::with_capacity(n);

	RNG.with(|rng| {
		balls.shuffle(&mut rng.borrow_mut());

		for _ in 0..n {
			vec.push(Card::new(&mut rng.borrow_mut(), true).into());
		}
	});

	(balls, vec)
}

fn play(n: usize) {
	let (balls, mut cards) = initialize(n);
	let mut accum = Vec::<usize>::with_capacity(n);
	let mut c = 0usize;

	'outer: for (r, b) in balls.iter().enumerate() {
		for card in cards.iter_mut() {
			if let Some(crd) = card {
				_ = crd.set(*b).unwrap();
				let expected = crd.remaining().view().iter().filter(|&&v| v == 0).count();
				let actual = crd.remaining().hit_count();
				assert_eq!(
					expected, actual,
					"Expected {} hits, got {}",
					expected, actual
				);

				if actual != 0 {
					accum.push(r + 1);
					*card = None;
					c += 1;

					if c == n {
						break 'outer;
					}
				}
			}
		}
	}

	assert!(cards.iter().all(|c| c.is_none()));

	ACCUM.with(|acc| acc.borrow_mut().set(&accum));
}
