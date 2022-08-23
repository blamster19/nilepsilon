use crate::algebra;
use crate::constants;

#[derive(Clone, PartialEq)]
pub enum BxDF {
	OrenNayar {
		a: algebra::Scalar,
		b: algebra::Scalar,
	},
}

impl BxDF {
	pub fn oren_nayar(sigma: algebra::Scalar) -> BxDF {
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
		}
	}

	// vectors in BSDF functions all point outward, but the outgoing vector is passed facing towards the shaded surface
	pub fn compute_bxdf (&self, incoming: algebra::Vector, outgoing: algebra::Vector, normal: algebra::Vector, lambda: algebra::Scalar) -> algebra::Scalar {
		match self {
			BxDF::OrenNayar { a, b } => {
				constants::PI_INV
			}
		}
	}

	pub fn pdf (&self, incoming: algebra::Vector, outgoing: algebra::Vector, normal: algebra::Vector, lambda: algebra::Scalar) -> algebra::Scalar {
		match self {
			BxDF::OrenNayar { a, b } => {
				incoming * normal * constants::PI_INV
			}
		}
	}
}
