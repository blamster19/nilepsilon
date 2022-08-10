use crate::algebra;
use rand::{thread_rng, rngs, Rng};
use rand::distributions::Uniform;
use rand_distr::{UnitSphere, Distribution};

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

	pub fn random_list_3d_sphere(&mut self, n: usize) -> Vec<(f64, f64, f64)> {
		let v: Vec<(f64, f64, f64)> = (0..n).map(|_| {
			let x: [f64; 3] = UnitSphere.sample(&mut self.rng);
			(x[0], x[1], x[2])
		}).collect();
		v
	}
}
