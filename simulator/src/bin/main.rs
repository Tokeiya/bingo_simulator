use owo_colors::colors::*;
use simulator::wrapper::ColorizeWrapper;

fn main() {
	let a = 10;
	let a = a.bg::<White>();

	println!("{}{}", a, 20.fg::<White>())
}
