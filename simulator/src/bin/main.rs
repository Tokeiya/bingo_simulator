use bingo::*;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

fn main() {
	foo();
}

fn bar() {
	let mut rnd = ChaCha20Rng::from_seed([42; _]);
	let mut accum = 0f64;

	for i in 0..20 {
		let (c, b) = hall(&mut rnd, 100);

		accum += c as f64;

		println!("round:{i} cnt:{c} tie:{b}");
	}

	println!("avg:{:.2}", accum / 100f64);
}
fn foo() {
	let mut rnd = ChaCha20Rng::from_seed([42; _]);
	let mut accum: [usize; 75] = [0; _];

	for _ in 0..10000 {
		accum[roll(&mut rnd)] += 1;
	}

	let mut ans = 0f64;
	for idx in 0..75 {
		ans += (accum[idx] as f64) * (idx as f64);
	}

	dbg!(ans);

	dbg!(accum);
	dbg!(ans as f64 / 1000.0);
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

	if cnt < 7 || cnt > 73 {
		println!("CNT:{cnt}");
		println!("{}\n\n", &card);
	}

	cnt
}

fn hall(rnd: &mut impl rand_core::Rng, num: usize) -> (usize, usize) {
	let mut vec: Vec<Card> = (0..num).map(|_| Card::new(rnd, true)).collect();
	let mut arr: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
	arr.shuffle(rnd);

	let mut cnt = 0;

	for ball in arr.into_iter() {
		cnt += 1;

		vec.iter_mut().for_each(|card| _ = card.set(ball).unwrap());
		let bingo = vec
			.iter()
			.filter(|x| x.remaining().view().contains(&0))
			.count();

		if bingo != 0 {
			return (cnt, bingo);
		}
	}

	unreachable!()
}
