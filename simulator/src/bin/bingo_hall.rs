use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() {
	let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());
}
