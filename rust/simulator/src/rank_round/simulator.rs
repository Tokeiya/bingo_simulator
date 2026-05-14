use super::accumulator::Accumulator;
use super::iter::Data;
use crate::rng_helper::generate;
use bingo::Card;
use dsv_writer::{NewLine, ToDsv};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

fn prepare(player: usize) -> (Vec<Option<Card>>, [u8; 75]) {
	thread_local! {
		static RNG:RefCell<ChaCha20Rng> = RefCell::new(generate());
	}

	let mut balls: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
	let mut vec: Vec<Option<Card>> = Vec::with_capacity(5);

	RNG.with(|rng| {
		balls.shuffle(&mut rng.borrow_mut());

		for _ in 0..player {
			let card = Card::new(&mut rng.borrow_mut(), true);
			vec.push(Some(card));
		}
	});

	(vec, balls)
}

pub fn do_simulate(
	output_path: &Path,
	initial_player: usize,
	step_player: usize,
	count: usize,
	iteration: usize,
) {
	let accum_arr: Vec<Accumulator> = (0..count)
		.map(|x| Accumulator::new(initial_player + (step_player * x), x, iteration))
		.collect();

	let ttl = iteration * count;

	(0usize..(count * iteration))
		.into_par_iter()
		.map(|x| Data::new(x, initial_player, step_player, iteration))
		.for_each(|data| {
			let (mut players, balls) = prepare(data.player);
			let accum = &accum_arr[(data.player / step_player) - 1];
			let mut cnt = 0usize;
			let mut result = Vec::<usize>::new();

			'outer: for (round, ball) in balls.into_iter().enumerate() {
				for player in players.iter_mut() {
					if let Some(p) = player {
						_ = p.set(ball);
						if p.remaining().hit_count() != 0 {
							result.push(round + 1);
							*player = None;
							cnt += 1;

							if cnt == data.player {
								break 'outer;
							}
						}
					}
				}
			}

			if data.play_id & 0xFF_FF == 0 {
				println!(
					"{data:?} {}/{ttl} {:.2} %",
					data.play_id,
					((data.play_id as f64) / (ttl as f64)) * 100f64
				)
			}

			accum.input(&result);
		});

	let file = std::fs::File::create(output_path).unwrap();
	let mut writer = dsv_writer::RawWriter::try_new(file, '\t', NewLine::Lf).unwrap();

	Accumulator::write_header(&mut writer).unwrap();
	for acc in accum_arr.iter() {
		acc.to_dsv(&mut writer).unwrap();
	}
}
