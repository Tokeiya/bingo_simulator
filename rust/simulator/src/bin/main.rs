use bingo::*;
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

fn main() {
	foo();
}

// fn bar() {
// 	let mut rnd = ChaCha20Rng::from_seed([42; _]);
// 	let mut accum = 0f64;
//
// 	for i in 0..20 {
// 		let (c, b) = hall(&mut rnd, 100);
//
// 		accum += c as f64;
//
// 		println!("round:{i} cnt:{c} tie:{b}");
// 	}
//
// 	println!("avg:{:.2}", accum / 100f64);
// }
fn foo() {
	const ROUND: usize = 100_000_000;
	let mut rnd = ChaCha20Rng::from_seed([42; _]);
	let mut accum: [usize; 75] = [0; _];

	for cnt in 0..ROUND {
		accum[roll(&mut rnd, cnt)] += 1;
	}

	let mut ans = 0f64;
	for idx in 0..75 {
		ans += (accum[idx] as f64) * (idx as f64);
	}

	dbg!(ans);

	dbg!(ans as f64 / ROUND as f64);

	for idx in 0..75 {
		println!("idx:{idx} cnt:{}", accum[idx]);
	}
}

fn roll(rnd: &mut impl rand_core::Rng, round: usize) -> usize {
	let mut arr: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
	arr.shuffle(rnd);

	let mut card = Card::new(rnd, true);
	let mut cnt = 0;

	for ball in arr.iter() {
		cnt += 1;

		card.set(*ball).unwrap();

		if card.remaining().view().contains(&0) {
			break;
		}
	}

	if cnt <= 4 || cnt >= 71 {
		println!("ROUND:{round} CNT:{cnt}");
		println!("{}", &card);
		print!("\n[");
		for i in arr.iter() {
			print!("{i}, ");
		}
		println!("]\n\n");
	}

	cnt
}

// fn hall(rnd: &mut impl rand_core::Rng, num: usize) -> (usize, usize) {
// 	let mut vec: Vec<Card> = (0..num).map(|_| Card::new(rnd, true)).collect();
// 	let mut arr: [u8; 75] = std::array::from_fn(|i| (i + 1) as u8);
// 	arr.shuffle(rnd);
//
// 	let mut cnt = 0;
//
// 	for ball in arr.into_iter() {
// 		cnt += 1;
//
// 		vec.iter_mut().for_each(|card| _ = card.set(ball).unwrap());
// 		let bingo = vec
// 			.iter()
// 			.filter(|x| x.remaining().view().contains(&0))
// 			.count();
//
// 		if bingo != 0 {
// 			return (cnt, bingo);
// 		}
// 	}
//
// 	unreachable!()
// }
