use bingo::Card;
use clap::Parser;
use dsv_writer::{NewLine, RawWriter, ToDsv};
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use simulator::aggregation::Table;
use simulator::play::play;
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
