use rand_chacha::ChaCha20Rng;
use rand_core::{Rng, SeedableRng};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

static STREAM_ID: AtomicU64 = AtomicU64::new(0);

static SEED: LazyLock<[u8; 32]> = LazyLock::new(|| {
	let mut arr = [0; 32];
	rand::rng().fill_bytes(&mut arr);
	arr
});

pub fn generate() -> ChaCha20Rng {
	let mut rng = ChaCha20Rng::from_seed(*SEED);
	let id = STREAM_ID.fetch_add(1, Ordering::Relaxed);
	rng.set_stream(id);
	rng
}
