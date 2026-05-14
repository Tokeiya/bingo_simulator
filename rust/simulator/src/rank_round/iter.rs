#[derive(Debug)]
pub struct Data {
	pub play_id: usize,
	pub player: usize,
	pub total_iteration: usize,
	pub current_iteration: usize,
}

pub struct SimulationIterator {
	play_id: usize,
	iteration: usize,
	current_iteration: usize,
	player_step: usize,
	player_limit: usize,
	player: usize,
}

impl SimulationIterator {
	pub fn new(
		initial_player: usize,
		player_count: usize,
		player_step: usize,
		iteration: usize,
	) -> Self {
		Self {
			play_id: 0,
			iteration,
			player_limit: initial_player + player_count * player_step,
			player_step,
			player: initial_player,
			current_iteration: 0,
		}
	}
}

impl Iterator for SimulationIterator {
	type Item = Data;

	fn next(&mut self) -> Option<Self::Item> {
		let dta = Data {
			play_id: self.play_id,
			player: self.player,
			total_iteration: self.iteration,
			current_iteration: self.current_iteration,
		};

		if self.player >= self.player_limit {
			return None;
		}

		self.play_id += 1;
		self.current_iteration += 1;

		if self.current_iteration >= self.iteration {
			self.player += self.player_step;
			self.current_iteration = 0;
		}

		Some(dta)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn next() {
		let mut fixture = SimulationIterator::new(10, 10, 10, 10);
		let mut count = 0usize;

		for player in (1..=10).map(|x| x * 10) {
			for i in 0..10 {
				let dta = fixture.next().unwrap();
				assert_eq!(dta.player, player);
				assert_eq!(dta.current_iteration, i);
				assert_eq!(dta.total_iteration, 10);
				assert_eq!(dta.play_id, count);
				count += 1;
			}
		}
	}
}
