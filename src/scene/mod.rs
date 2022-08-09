use crate::algebra;
use crate::camera;
use crate::constants;
use crate::primitives;

pub enum BackgroundType {
	BlackBody(algebra::Scalar),
	BlackBodyNormalized(algebra::Scalar),
}

pub struct Background {
	pub color: BackgroundType,
	pub radiance: algebra::Scalar,
}

impl Background {
	pub fn return_radiance(
		&self,
		dir: algebra::Vector,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		match self.color {
			BackgroundType::BlackBody(temperature) => {
				constants::TWO_HC2
					/ (lambda.powi(5) * ((constants::HC_BY_K / lambda / temperature).exp() - 1.0))
			}
			BackgroundType::BlackBodyNormalized(temperature) => {
				let lmax: algebra::Scalar = constants::WIEN / temperature;
				constants::TWO_HC2
					/ (lambda.powi(5) * ((constants::HC_BY_K / lambda / temperature).exp() - 1.0))
					/ (
				constants::TWO_HC2
					/ (lmax.powi(5) * ((constants::HC_BY_K / lmax / temperature).exp() - 1.0))
				) * self.radiance
			}
		}
	}
}

pub struct Scene {
	pub objects: Vec<primitives::Primitive>,
	pub camera: camera::Camera,
	pub background: Background,
}
