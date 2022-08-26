use crate::algebra;
use crate::constants;
use crate::ray;
use crate::shaders;

type Color = Vec<algebra::Scalar>;

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
	Dielectric { sigma: algebra::Scalar },
}

#[derive(Clone, PartialEq)]
pub struct Material {
	pub emitter: EmissionType,
	bxdf: shaders::BxDF,
	pub color: Color,
}

impl Material {
	pub fn new(emitter: EmissionType, surface: SurfaceType, color: Color) -> Self {
		Self {
			emitter,
			color,
			bxdf: match surface {
				SurfaceType::Dielectric { sigma } => shaders::BxDF::oren_nayar(sigma),
			},
		}
	}

	pub fn return_scatter_radiance(
		&self,
		incoming: algebra::Vector,
		outgoing: algebra::Vector,
		normal: algebra::Vector,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		self.return_color(lambda) * self.bxdf.compute_bxdf(incoming, outgoing, normal, lambda)
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
		outgoing: algebra::Vector,
		normal: algebra::Vector,
		random: (f64, f64),
	) -> algebra::Vector {
		let r1 = random.0;
		let r2 = random.1;
		let z = (1.0 - r2).sqrt();
		let phi = 2.0 * constants::PI * r1;
		algebra::Vector::new(phi.cos() * r2.sqrt(), phi.sin() * r2.sqrt(), z)
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

	fn return_color(&self, lambda: algebra::Scalar) -> algebra::Scalar {
		let mut color: algebra::Scalar = 0.0;
		for (power, coefficient) in self.color.iter().enumerate() {
			color += coefficient * lambda.powi(power.try_into().unwrap());
		}
		if color > 1.0 {
			return 1.0;
		} else if color < 0.0 {
			return 0.0;
		} else {
			return color;
		}
	}
}
