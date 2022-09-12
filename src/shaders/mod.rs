use crate::algebra;
use crate::constants;
use std::cmp;

pub type Color = Vec<algebra::Scalar>;

pub enum Lobe {
	Cosine,
	DeltaReflect,
	GGX_reflect { alpha: algebra::Scalar },
	DeltaRefract,
}

#[derive(Clone, PartialEq)]
pub enum BxDF {
	OrenNayar {
		a: algebra::Scalar,
		b: algebra::Scalar,
		color: Color,
	},
	Specular {},
	SpecularRefract {},
	GGX_reflect {
		alpha: algebra::Scalar,
		alpha2: algebra::Scalar,
	},
}

impl BxDF {
	pub fn oren_nayar(sigma: algebra::Scalar, color: Color) -> BxDF {
		let sigma2;
		if sigma * sigma > 1.0 {
			sigma2 = 1.0;
		} else if sigma * sigma < 0.0 {
			sigma2 = 0.0;
		} else {
			sigma2 = sigma * sigma;
		}
		BxDF::OrenNayar {
			a: 1.0 - 0.5 * sigma2 / (sigma2 + 0.33),
			b: 0.45 * sigma2 / (sigma2 + 0.09),
			color,
		}
	}

	pub fn specular() -> BxDF {
		BxDF::Specular {}
	}

	pub fn specular_refract() -> BxDF {
		BxDF::SpecularRefract {}
	}

	pub fn ggx_reflect(roughness: algebra::Scalar) -> BxDF {
		BxDF::GGX_reflect {
			alpha: roughness.powi(2),
			alpha2: roughness.powi(4),
		}
	}

	pub fn lobe(&self) -> Lobe {
		match self {
			BxDF::OrenNayar { .. } => Lobe::Cosine,
			BxDF::Specular {} => Lobe::DeltaReflect,
			BxDF::SpecularRefract { .. } => Lobe::DeltaRefract,
			BxDF::GGX_reflect { alpha, .. } => Lobe::GGX_reflect { alpha: *alpha },
		}
	}

	pub fn compute_bxdf(
		&self,
		incoming: algebra::Vector,
		outgoing: algebra::Vector,
		normal: algebra::Vector,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		match self {
			BxDF::OrenNayar { a, b, color } => {
				let max = |a, b| if a - b > 0.0 { a } else { b };
				let clamp = |x, a, b| {
					if x < a {
						a
					} else if x > b {
						b
					} else {
						x
					}
				};
				let mut max_cos: algebra::Scalar = 0.0;
				let cos_theta_i = incoming.z;
				let cos_theta_o = outgoing.z;
				let sin_theta_i = max(0.0, 1.0 - cos_theta_i.powi(2)).sqrt();
				let sin_theta_o = max(0.0, 1.0 - cos_theta_o.powi(2)).sqrt();
				if sin_theta_i > 1e-4 && sin_theta_o > 1e-4 {
					let cos_phi_i = if sin_theta_i == 0.0 {
						1.0
					} else {
						clamp(incoming.x / sin_theta_i, -1.0, 1.0)
					};
					let cos_phi_o = if sin_theta_o == 0.0 {
						1.0
					} else {
						clamp(outgoing.x / sin_theta_o, -1.0, 1.0)
					};
					let sin_phi_i = if sin_theta_i == 0.0 {
						0.0
					} else {
						clamp(incoming.y / sin_theta_i, -1.0, 1.0)
					};
					let sin_phi_o = if sin_theta_o == 0.0 {
						0.0
					} else {
						clamp(outgoing.y / sin_theta_o, -1.0, 1.0)
					};
					let d_cos = cos_phi_i * cos_phi_o + sin_phi_i * sin_phi_o;
					max_cos = max(0.0, d_cos);
				}
				let sin_alpha: algebra::Scalar;
				let tan_beta: algebra::Scalar;

				if cos_theta_i.abs() > cos_theta_o.abs() {
					sin_alpha = sin_theta_o;
					tan_beta = sin_theta_i / cos_theta_i.abs();
				} else {
					sin_alpha = sin_theta_i;
					tan_beta = sin_theta_o / cos_theta_o.abs();
				}
				return self.return_color(&color, lambda)
					* constants::PI_INV * (a + b * max_cos * sin_alpha * tan_beta);
			}
			BxDF::Specular {} => 1.0,
			BxDF::GGX_reflect { alpha2, .. } => {
				let n1: algebra::Scalar = 1.0;
				let n2: algebra::Scalar = 1.45;
				let half_vec = (incoming + outgoing).normalize();
				let denom = (incoming * normal) * (outgoing * normal);
				0.25 * self.d_ggx(half_vec, normal, *alpha2)
					* self.g_ggx(incoming, outgoing, half_vec, normal, *alpha2)
					/ denom
			}
			BxDF::SpecularRefract {} => 1.0,
		}
	}

	pub fn pdf(
		&self,
		incoming: algebra::Vector,
		outgoing: algebra::Vector,
		normal: algebra::Vector,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		match self {
			BxDF::OrenNayar { .. } => (normal * incoming).abs() * constants::PI_INV,
			BxDF::Specular {} => 1.0,
			BxDF::GGX_reflect { alpha2, .. } => {
				let half_vec = (incoming + outgoing).normalize();
				let clamp = |x| if x < 0.0 { 0.0 } else { x };
				self.d_ggx(half_vec, normal, *alpha2)
					* (normal * incoming).abs()
					* constants::PI_INV
			}
			BxDF::SpecularRefract {} => 1.0,
		}
	}
	fn return_color(&self, c: &Color, lambda: algebra::Scalar) -> algebra::Scalar {
		let mut color: algebra::Scalar = 0.0;
		for (power, coefficient) in c.iter().enumerate() {
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

	fn d_ggx(
		&self,
		direction: algebra::Vector,
		normal: algebra::Vector,
		alpha2: algebra::Scalar,
	) -> algebra::Scalar {
		let cos2_theta: algebra::Scalar = (normal * direction).powi(2);
		let denom = (cos2_theta * (alpha2 - 1.0) + 1.0).powi(2);
		alpha2 * constants::PI_INV / denom
	}

	fn g_ggx(
		&self,
		incoming: algebra::Vector,
		outgoing: algebra::Vector,
		half_vec: algebra::Vector,
		normal: algebra::Vector,
		alpha2: algebra::Scalar,
	) -> algebra::Scalar {
		self.gp_ggx(incoming, half_vec, normal, alpha2)
			* self.gp_ggx(outgoing, half_vec, normal, alpha2)
	}

	fn gp_ggx(
		&self,
		direction: algebra::Vector,
		half_vec: algebra::Vector,
		normal: algebra::Vector,
		alpha2: algebra::Scalar,
	) -> algebra::Scalar {
		let clamp = |x| if x < 0.0 { 0.0 } else { x };
		let cos_theta = clamp(normal * direction);
		let sqrterm = alpha2 + (1.0 - alpha2) * cos_theta.powi(2);
		let denom = cos_theta + sqrterm.sqrt();
		2.0 * cos_theta / denom
	}
	pub fn fresnel_schlick_dielectric(
		&self,
		n_1: algebra::Scalar,
		n_2: algebra::Scalar,
		outgoing: algebra::Vector,
		half_vec: algebra::Vector,
	) -> algebra::Scalar {
		let cos_theta = half_vec * outgoing;
		let f_0 = ((n_1 - n_2) / (n_1 + n_2)).powi(2);
		f_0 + (1.0 - f_0) * (1.0 - cos_theta).powi(5)
	}

	pub fn fresnel_conductor(
		&self,
		n: algebra::Scalar,
		k: algebra::Scalar,
		outgoing: algebra::Vector,
		half_vec: algebra::Vector,
	) -> algebra::Scalar {
		let cos_theta = outgoing * half_vec;
		let denom = (n + 1.0).powi(2) + k * k;
		((n - 1.0).powi(2) + 4.0 * n * (1.0 - cos_theta).powi(5) + k * k) / denom
	}
}

impl Lobe {
	pub fn evaluate_lobe(
		lobe: Self,
		theta_i: algebra::Scalar,
		phi_i: algebra::Scalar,
		random_dir: (f64, f64),
	) -> (algebra::Scalar, algebra::Scalar) {
		match lobe {
			Self::Cosine => (
				random_dir.0.sqrt().acos(),
				random_dir.1 * 2.0 * constants::PI,
			),
			Self::DeltaReflect => (theta_i, phi_i + constants::PI),
			Self::GGX_reflect { alpha } =>
					(
						(alpha * (random_dir.0 / (1.0 - random_dir.0)).sqrt()).atan(),
						random_dir.1 * 2.0 * constants::PI,
					),
			Self::DeltaRefract => {
				let n1 = 1.0;
				let n2 = 1.5;
				return if theta_i < 0.5 * constants::PI {
					let ratio = n1 * theta_i.sin() / n2;
					if ratio >= -1.0 && ratio <= 1.0 {
						(constants::PI - ratio.asin(), phi_i + constants::PI)
					} else {
						(theta_i, phi_i + constants::PI)
					}
				} else {
					let ratio = n2 * (constants::PI - theta_i).sin() / n1;
					if ratio >= -1.0 && ratio <= 1.0 {
						(ratio.asin(), phi_i + constants::PI)
					} else {
						(theta_i, phi_i + constants::PI)
					}
				};
			}
		}
	}

}
