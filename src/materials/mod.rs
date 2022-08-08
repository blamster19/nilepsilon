use crate::algebra;
use crate::ray;

#[derive(Clone)]
pub struct Material {
	pub emitter: bool,
}

impl Material {
	pub fn return_radiance() -> algebra::Scalar {
		1.0
	}
}
