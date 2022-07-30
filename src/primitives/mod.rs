use crate::algebra;
use crate::ray;

enum Intersection {
	Hit(algebra::Vector),
	Miss(algebra::Vector),
}

pub enum Primitive {
	Sphere {
		position: algebra::Vector,
		radius: algebra::Scalar,
	}
}

impl Primitive {
	pub fn intersect(&self, ray: ray::Ray) -> bool {
		match self {
			Primitive::Sphere { position, radius } => {
				let orig_to_center: algebra::Vector = ray.orig - *position;
				let a = ray.dir * ray.dir;
				let b = 2.0 * orig_to_center * ray.dir;
				let c = orig_to_center.norm_sqr() - radius * radius;
				let delta = b * b - 4.0 * a * c;
				delta > 0.0
			}
		}
	}
}
