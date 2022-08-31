use crate::algebra;
use crate::constants;
use crate::ray;
use crate::shaders;

#[derive(Clone, PartialEq)]
pub enum EmissionType {
	NonEmissive,
	Incandescent {
		temperature: algebra::Scalar,
	},
	Cool {
		temperature: algebra::Scalar,
		power: algebra::Scalar,
	},
}

#[derive(Clone, PartialEq)]
pub enum SurfaceType {
	Dielectric { sigma: algebra::Scalar, color: shaders::Color },
	Conductor {},
}

#[derive(Clone, PartialEq)]
pub struct Material {
	pub emitter: EmissionType,
	bxdf: shaders::BxDF,
}

impl Material {
	pub fn new(emitter: EmissionType, surface: SurfaceType) -> Self {
		Self {
			emitter,
			bxdf: match surface {
				SurfaceType::Dielectric { sigma, color } => shaders::BxDF::oren_nayar(sigma, color),
				SurfaceType::Conductor {} => shaders::BxDF::specular(),
			},
		}
	}

	pub fn return_scatter_radiance(
		&self,
		theta_i: algebra::Scalar,
		phi_i: algebra::Scalar,
		theta_o: algebra::Scalar,
		phi_o: algebra::Scalar,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		self.bxdf
			.compute_bxdf(theta_i, phi_i, theta_o, phi_o, lambda)
	}

	pub fn return_emission_radiance(&self, lambda: algebra::Scalar) -> algebra::Scalar {
		match self.emitter {
			EmissionType::NonEmissive => 0.0,
			EmissionType::Incandescent { temperature } => {
				constants::TWO_HC2
					/ (lambda.powi(5) * ((constants::HC_BY_K / lambda / temperature).exp() - 1.0))
			}
			EmissionType::Cool { temperature, power } => {
				let lmax: algebra::Scalar = constants::WIEN / temperature;
				constants::TWO_HC2
					/ (lambda.powi(5) * ((constants::HC_BY_K / lambda / temperature).exp() - 1.0))
					/ (constants::TWO_HC2
						/ (lmax.powi(5) * ((constants::HC_BY_K / lmax / temperature).exp() - 1.0)))
					* power
			}
		}
	}

	pub fn return_direction(
		&self,
		theta_i: algebra::Scalar,
		phi_i: algebra::Scalar,
		random: (f64, f64),
	) -> (algebra::Scalar, algebra::Scalar) {
		match self.bxdf.lobe() {
			shaders::Lobe::Cosine => (
				random.0 * 0.5 * constants::PI,
				random.1 * 2.0 * constants::PI
			),
			shaders::Lobe::Delta => (theta_i, phi_i + constants::PI),
		}
	}

	pub fn return_pdf(
		&self,
		incoming: algebra::Vector,
		outgoing: algebra::Vector,
		normal: algebra::Vector,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		self.bxdf.pdf(incoming, outgoing, normal, lambda)
	}

	pub fn new_basis(&self, normal: algebra::Vector) -> algebra::Basis {
		let a: algebra::Vector;
		if normal.x.abs() > 1.0 - algebra::Scalar::EPSILON {
			a = algebra::Vector::new(0.0, 1.0, 0.0);
		} else {
			a = algebra::Vector::new(1.0, 0.0, 0.0);
		}
		let x = normal % a;
		let y = normal % x;
		algebra::Basis::new(x.normalize(), y.normalize(), normal)
	}
}
