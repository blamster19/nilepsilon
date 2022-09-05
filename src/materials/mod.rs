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
	DielectricOpaque {
		color: shaders::Color,
		roughness: algebra::Scalar,
	},
	DielectricTransparent {
		roughness: algebra::Scalar,
	},
	Conductor {
		roughness: algebra::Scalar,
	},
}

#[derive(Clone, PartialEq)]
enum InternalType {
	DielOpaq,
	DielTrs,
	Cond,
}

#[derive(Clone, PartialEq)]
pub struct Material {
	pub emitter: EmissionType,
	bxdf: Vec<shaders::BxDF>,
	n: algebra::Scalar,
	k: algebra::Scalar,
	surface: InternalType,
}

impl Material {
	pub fn new(
		emitter: EmissionType,
		surface: SurfaceType,
		n: algebra::Scalar,
		k: algebra::Scalar,
	) -> Self {
		match surface {
			SurfaceType::Conductor { roughness } => Self {
				emitter,
				bxdf: vec![shaders::BxDF::ggx_reflect(roughness)],
				n,
				k,
				surface: InternalType::Cond,
			},
			SurfaceType::DielectricOpaque { color, roughness } => Self {
				emitter,
				bxdf: vec![
					shaders::BxDF::oren_nayar(0.5 * roughness, color),
					shaders::BxDF::ggx_reflect(roughness),
				],
				n,
				k,
				surface: InternalType::DielOpaq,
			},
			SurfaceType::DielectricTransparent { roughness } => Self {
				emitter,
				bxdf: vec![
					shaders::BxDF::specular_refract(),
					shaders::BxDF::ggx_reflect(roughness),
				],
				n,
				k,
				surface: InternalType::DielTrs,
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
		match self.surface {
			InternalType::DielOpaq => {
				let diff = self.bxdf[0].compute_bxdf(incoming, outgoing, normal, lambda);
				let glos = self.bxdf[1].compute_bxdf(incoming, outgoing, normal, lambda);
				let half_vec = (incoming + outgoing).normalize();
				let f = self.bxdf[0].fresnel_schlick_dielectric(1.0, self.n, outgoing, half_vec);
				diff * (1.0 - f) + glos * f
			}
			InternalType::DielTrs => {
				return self.bxdf[0].compute_bxdf(incoming, outgoing, normal, lambda);
			}
			InternalType::Cond => {
				let glos = self.bxdf[0].compute_bxdf(incoming, outgoing, normal, lambda);
				let half_vec = (incoming + outgoing).normalize();
				let f = self.bxdf[0].fresnel_conductor(self.n, self.k, outgoing, half_vec);
				glos * f
			}
		}
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
		match self.bxdf[0].lobe() {
			shaders::Lobe::Cosine => (random.0.sqrt().acos(), random.1 * 2.0 * constants::PI),
			shaders::Lobe::DeltaReflect => (theta_i, phi_i + constants::PI),
			shaders::Lobe::GGX_reflect => {
				if let shaders::BxDF::GGX_reflect { alpha, .. } = self.bxdf[0] {
					return (
						(alpha * (random.0 / (1.0 - random.0)).sqrt()).atan(),
						random.1 * 2.0 * constants::PI,
					);
				} else {
					return (theta_i, phi_i + constants::PI);
				}
			}
			shaders::Lobe::DeltaRefract => {
				let n1 = 1.0;
				let n2 = 1.5;
				return if theta_i < 0.5 * constants::PI {
					(
						constants::PI - (n1 * theta_i.sin() / n2).asin(),
						phi_i + constants::PI,
					)
				} else {
					((n2 * theta_i.sin() / n1).asin(), phi_i + constants::PI)
				};
			}
		}
	}

	pub fn return_pdf(
		&self,
		incoming: algebra::Vector,
		outgoing: algebra::Vector,
		normal: algebra::Vector,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		self.bxdf[0].pdf(incoming, outgoing, normal, lambda)
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
