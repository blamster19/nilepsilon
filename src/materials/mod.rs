use crate::algebra;
use crate::constants;
use crate::ray;

#[derive(Clone)]
pub enum EmissionType {
	NonEmissive,
	Incandescent{ temperature: algebra::Scalar },
	Cool{ temperature: algebra::Scalar, power: algebra::Scalar },
}

#[derive(Clone)]
pub enum SurfaceType {
	Dielectric{ }
}

#[derive(Clone)]
pub struct Material {
	pub emitter: EmissionType,
	pub surface: SurfaceType,
}

impl Material {
	pub fn return_scatter_radiance(&self, incoming: algebra::Vector, outgoing: algebra::Vector, lambda: algebra::Scalar) -> algebra::Scalar {
		1.0
	}

	pub fn return_emission_radiance(&self, lambda: algebra::Scalar) -> algebra::Scalar {
		match self.emitter {
			EmissionType::NonEmissive => 0.0,
			EmissionType::Incandescent{ temperature } => {
				constants::TWO_HC2
					/ (lambda.powi(5) * ((constants::HC_BY_K / lambda / temperature).exp() - 1.0))
			}
			EmissionType::Cool{ temperature, power } => {
				let lmax: algebra::Scalar = constants::WIEN / temperature;
				constants::TWO_HC2
					/ (lambda.powi(5) * ((constants::HC_BY_K / lambda / temperature).exp() - 1.0))
					/ (constants::TWO_HC2
						/ (lmax.powi(5) * ((constants::HC_BY_K / lmax / temperature).exp() - 1.0)))
					* power
			}
		}
	}
}
