use crate::algebra;

#[derive(Default, Clone, Debug, Copy)]
pub struct Ray {
	pub orig: algebra::Vector,
	pub dir: algebra::Vector,
}

impl Ray {
	pub fn new(orig: algebra::Vector, dir: algebra::Vector) -> Ray {
		Ray { orig, dir }
	}

	pub fn point_on_line(&self, t: algebra::Scalar) -> algebra::Vector {
		self.orig + t * self.dir
	}
}
