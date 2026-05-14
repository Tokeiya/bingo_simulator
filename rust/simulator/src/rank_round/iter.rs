#[derive(Debug)]
pub struct Data {
	pub play_id: usize,
	pub player: usize,
	pub total_iteration: usize,
	pub current_iteration: usize,
}

impl Data {
	pub fn new(play_id: usize, initial_player: usize, step: usize, total_iteration: usize) -> Self {
		let iteration = play_id % total_iteration;
		let player = initial_player + (play_id / total_iteration) * step;
		Self {
			play_id,
			player,
			total_iteration,
			current_iteration: iteration,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn data() {
		let mut cnt = 0;

		for p in (1..=10).map(|x| x * 10) {
			for i in 0..10 {
				let fixture = Data::new(cnt, 10, 10, 10);
				dbg!(&fixture);
				assert_eq!(fixture.player, p);
				assert_eq!(fixture.current_iteration, i);
				assert_eq!(fixture.total_iteration, 10);
				assert_eq!(fixture.play_id, cnt);
				cnt += 1;
			}
		}
	}
}
