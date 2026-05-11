use bingo::*;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
	#[arg(long, default_value = "../data")]
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

fn main() {}
