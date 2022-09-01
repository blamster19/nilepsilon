use crate::algebra;
use crate::constants;
use std::cmp;

pub type Color = Vec<algebra::Scalar>;

pub enum Lobe {
	Cosine,
	Delta,
}

#[derive(Clone, PartialEq)]
pub enum BxDF {
	OrenNayar {
		a: algebra::Scalar,
		b: algebra::Scalar,
		color: Color,
	},
	Specular {},
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

	pub fn lobe(&self) -> Lobe {
		match self {
			BxDF::OrenNayar { .. } => Lobe::Cosine,
			BxDF::Specular {} => Lobe::Delta,
		}
	}

	// vectors in BSDF functions all point outward, but the outgoing vector is passed facing towards the shaded surface
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
				let clamp = |x, a, b| if x < a { a } else if x > b { b } else { x };
				let mut max_cos: algebra::Scalar = 0.0;
				let cos_theta_i = incoming.z;
				let cos_theta_o = outgoing.z;
				let sin_theta_i = max(0.0, 1.0 - cos_theta_i.powi(2)).sqrt();
				let sin_theta_o = max(0.0, 1.0 - cos_theta_o.powi(2)).sqrt();
				if sin_theta_i > 1e-4 && sin_theta_o > 1e-4 {
					let cos_phi_i = if sin_theta_i == 0.0 { 1.0 } else { clamp(incoming.x / sin_theta_i, -1.0, 1.0) };
					let cos_phi_o = if sin_theta_o == 0.0 { 1.0 } else { clamp(outgoing.x / sin_theta_o, -1.0, 1.0) };
					let sin_phi_i = if sin_theta_i == 0.0 { 0.0 } else { clamp(incoming.y / sin_theta_i, -1.0, 1.0) };
					let sin_phi_o = if sin_theta_o == 0.0 { 0.0 } else { clamp(outgoing.y / sin_theta_o, -1.0, 1.0) };
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
			BxDF::Specular {} => 1.0
		}
	}

	pub fn pdf(
		&self,
		theta_i: algebra::Scalar,
		phi_i: algebra::Scalar,
		theta_o: algebra::Scalar,
		phi_o: algebra::Scalar,
		lambda: algebra::Scalar,
	) -> algebra::Scalar {
		match self {
			BxDF::OrenNayar { .. } => theta_i.cos() * constants::PI_INV,
			BxDF::Specular {} => 1.0,
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
}
