use rand_core::Rng;

pub struct Card([i8; 25]);

impl Card {
	pub fn new(rnd: &mut impl Rng) -> Self {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use colored::Colorize;
	#[test]
	fn hoge() {
		let a = "hello";

		println!("{}", a.blue());
	}
}
