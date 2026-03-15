use std::fmt::Display;

pub trait Sample{
	fn sample_func(&self);
}

impl<T:Display> Sample for T {
	fn sample_func(&self) {
		println!("sample:{}", self);
	}
}