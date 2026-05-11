use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

const RESOLUTION: usize = 0x0f_ff_ff;

struct Report {
	resolution: usize,
	total: usize,
	current: AtomicUsize,
}

impl Report {
	pub fn new(resolution: usize, total: usize) -> Self {
		Self {
			resolution,
			total,
			current: AtomicUsize::new(0),
		}
	}

	pub fn inclement(&self) {
		let current = self.current.fetch_add(self.resolution, Ordering::Relaxed) + self.resolution;
		println!(
			"{}/{} {:.2} %",
			current,
			self.total,
			(current as f64 / self.total as f64) * 100.0
		);
	}
}
fn main() {
	let report = Report::new(RESOLUTION, 1_000_000 * 4000);

	(0..4000usize).into_par_iter().for_each(|_| {
		for i in 0..1_000_000 {
			if i & RESOLUTION == 0 {
				report.inclement();
			}
		}
	});
}
