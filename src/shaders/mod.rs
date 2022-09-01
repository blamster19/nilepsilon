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
				let alpha: algebra::Scalar;
				let beta: algebra::Scalar;
				if theta_i - theta_o > 0.0 {
					alpha = theta_i;
					beta = theta_o;
				} else {
					alpha = theta_o;
					beta = theta_i;
				}
				let mut cos_phi = (phi_i - phi_o).cos();
				if cos_phi < 0.0 {
					cos_phi = 0.0;
				}
				return self.return_color(&color, lambda)
					* constants::PI_INV * (a + b * cos_phi * alpha.sin() * beta.tan());
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
