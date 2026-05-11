use bingo::Card;
use clap::Parser;
use dsv_writer::{NewLine, RawWriter, ToDsv};
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use simulator::aggregation::Table;
use simulator::rng_helper::generate;
use std::cell;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
	#[arg(short = 'i', long)]
	iteration: usize,
	#[arg(short = 'p', long)]
	player: usize,
	#[arg(short, long, default_value = "../data/rank_round_iteration.tsv")]
	path: String,
}

impl Args {
	pub fn create_path(&self, channel: usize) -> PathBuf {
		Path::new(&self.path).join(format!("{:00}_{channel:00}.tsv", self.player))
	}
}

static WRITER: OnceLock<Mutex<RawWriter<File>>> = OnceLock::new();

fn main() {
	let args = Args::parse();

	WRITER
		.set(Mutex::new(
			RawWriter::try_new(
				std::fs::File::create(&args.path).unwrap(),
				'\t',
				NewLine::Lf,
			)
			.unwrap(),
		))
		.unwrap();

	(0..20usize).into_par_iter().for_each(|channel| {
		let mut rng = generate();
		let mut table = Table::new(args.player, args.iteration);

		for i in 0..args.iteration {
			play(args.player, &mut rng, &mut table);

			if i & 0xffff == 0 {
				println!("{channel} {i} / {} completed", args.iteration)
			}
		}

		let mut w = WRITER.get().unwrap().lock().unwrap();
		_ = table.to_dsv(&mut *w);
	});
}

fn initialize(n: usize, rng: &mut impl rand::Rng) -> ([u8; 75], Vec<Option<Card>>) {
	let mut balls = std::array::from_fn::<_, 75, _>(|i| (i + 1) as u8);
	let mut vec: Vec<Option<Card>> = Vec::with_capacity(n);

	balls.shuffle(rng);

	for _ in 0..n {
		vec.push(Card::new(rng, true).into());
	}

	(balls, vec)
}

fn play(n: usize, rng: &mut impl rand::Rng, table: &mut Table) {
	let (balls, mut cards) = initialize(n, rng);
	let mut accum = Vec::<usize>::with_capacity(n);
	let mut c = 0usize;

	'outer: for (r, b) in balls.iter().enumerate() {
		for card in cards.iter_mut() {
			if let Some(crd) = card {
				_ = crd.set(*b).unwrap();
				let expected = crd.remaining().view().iter().filter(|&&v| v == 0).count();
				let actual = crd.remaining().hit_count();
				debug_assert_eq!(
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

	debug_assert!(cards.iter().all(|c| c.is_none()));
	table.set(&accum);
}
