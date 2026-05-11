use crate::aggregation::Table;
use bingo::Card;
use rand::prelude::SliceRandom;

pub fn play(n: usize, rng: &mut impl rand::Rng, table: &mut Table) {
	fn initialize(n: usize, rng: &mut impl rand::Rng) -> ([u8; 75], Vec<Option<Card>>) {
		let mut balls = std::array::from_fn::<_, 75, _>(|i| (i + 1) as u8);
		let mut vec: Vec<Option<Card>> = Vec::with_capacity(n);

		balls.shuffle(rng);

		for _ in 0..n {
			vec.push(Card::new(rng, true).into());
		}

		(balls, vec)
	}

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
