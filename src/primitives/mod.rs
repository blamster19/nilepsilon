use crate::algebra;
use crate::ray;

enum Intersection {
	Hit(algebra::Vector),
	Miss(algebra::Vector),
}

pub struct Sphere {
	position: algebra::Vector,
	radius: algebra::Scalar,
}

impl Sphere {
	pub fn intersect(&self, ray: ray::Ray) -> bool {
		let orig_to_center: algebra::Vector = ray.orig - self.position;
		let a = ray.dir * ray.dir;
		let b = 2.0 * orig_to_center * ray.dir;
		let c = orig_to_center.norm_sqr() - self.radius * self.radius;
		let delta = b * b - 4.0 * a * c;
		delta > 0.0
	}
}
