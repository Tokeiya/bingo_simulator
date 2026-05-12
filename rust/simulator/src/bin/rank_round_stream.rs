use clap::Parser;
use dsv_writer::{NewLine, RawWriter, ToDsv};
use rayon::prelude::*;
use simulator::aggregation::Table;
use simulator::play::play;
use simulator::rng_helper::generate;
use std::fs::File;
use std::sync::{Mutex, OnceLock};

#[derive(Parser, Debug)]
pub struct Args {
	#[arg(long, default_value = "../data/rank_round_stream.tsv")]
	path: String,
	#[arg(long, short)]
	begin: usize,
	#[arg(long, short)]
	step: usize,
	#[arg(long, short)]
	count: usize,
	#[arg(long, short, default_value = "1_000_000")]
	iteration: usize,
}

static WRITER: OnceLock<Mutex<RawWriter<File>>> = OnceLock::new();

fn main() {
	let args = Args::parse();

	WRITER
		.set(Mutex::new(
			RawWriter::try_new(File::create(args.path).unwrap(), '\t', NewLine::Lf).unwrap(),
		))
		.unwrap();

	(0..args.count).into_par_iter().for_each(|i| {
		println!("{i} process start.");

		let cards = args.begin + (i * args.step);
		let mut rng = generate();
		let mut table = Table::new(cards, args.iteration);

		for _ in 0..args.iteration {
			play(cards, &mut rng, &mut table);
		}

		let mut w = WRITER.get().unwrap().lock().unwrap();
		table.to_dsv(&mut *w).unwrap();
	})
}
