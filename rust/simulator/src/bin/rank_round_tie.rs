use clap::Parser;
use simulator::rank_round::simulator::do_simulate;
use std::path::Path;

#[derive(Parser)]
struct Args {
	#[arg(long,short,default_value = None,help = "output file path")]
	path: Option<String>,
	#[arg(long, short, help = "initial player count")]
	begin: usize,
	#[arg(long, short, help = "inclement player count")]
	step: usize,
	#[arg(long, short, help = "play count")]
	count: usize,
	#[arg(long, short, default_value = "1000000", help = "play iteration count")]
	iteration: usize,
}
fn main() {
	let mut args = Args::parse();

	if args.path.is_none() {
		args.path = Some(format!(
			"../data/source/rank_round_{}_{}.tsv",
			args.begin,
			args.step * args.count
		));
	}

	do_simulate(
		Path::new(args.path.unwrap().as_str()),
		10,
		10,
		10,
		1_000_000,
	);
}
