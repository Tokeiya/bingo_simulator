use bingo::*;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

fn main() {
	let mut accum = [0usize; 75];
	let mut rnd = ChaCha20Rng::from_seed([42; _]);

	for i in 0..10000 {
		accum[roll(&mut rnd)] += 1;
	}

	for i in 0..accum.len() {
		println!("{}:{}", i, accum[i]);
	}
}

fn debug(rnd: &mut impl rand_core::Rng) {
	let mut arr: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
	arr.shuffle(rnd);

	let mut card = Card::new(rnd, true);
	println!("{:?}", card.remaining().view());

	card.set(11).unwrap();
	card.set(16).unwrap();
	card.set(49).unwrap();
	card.set(70).unwrap();

	println!("{}", card);
	println!("{:?}", card.remaining().view());
}

fn roll(rnd: &mut impl rand_core::Rng) -> usize {
	let mut arr: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
	arr.shuffle(rnd);

	let mut card = Card::new(rnd, true);
	let mut cnt = 0;

	for ball in arr.into_iter() {
		cnt += 1;

		card.set(ball).unwrap();

		if card.remaining().view().contains(&0) {
			break;
		}
	}

	if cnt < 10 || cnt > 65 {
		println!("CNT:{cnt}");
		println!("{}\n\n", &card);
	}

	cnt
}
