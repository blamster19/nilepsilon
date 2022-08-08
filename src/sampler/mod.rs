use crate::algebra;
use rand::{thread_rng, rngs, Rng};
use rand::distributions::Uniform;

pub struct Sampler {
	pub rng: rand::rngs::ThreadRng,
}

impl Sampler {
	pub fn init(&mut self) {
		self.rng = thread_rng();
	}

	pub fn random_list_1d(&mut self, n: usize, min: f64, max: f64) -> Vec<f64> {
		let v: Vec<f64> = (0..n).map(|_| {
			self.rng.gen_range(min..max)
		}).collect();
		v
	}

	pub fn random_list_2d(&mut self, n: usize, min: f64, max: f64) -> Vec<(f64, f64)> {
		let v: Vec<(f64, f64)> = (0..n).map(|_| {
			(self.rng.gen_range(min..max), self.rng.gen_range(min..max))
		}).collect();
		v
	}
}
