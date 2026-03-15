use bingo::wrapper::ColorizeWrapper;

fn main() {
	println!(
		"{} {}",
		"01".on_green().black().bold(),
		"03".on_yellow().black().bold()
	)
}
