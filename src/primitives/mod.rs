use crate::algebra;
use crate::ray;

#[derive(Clone, Copy)]
pub enum Primitive {
	Sphere {
		position: algebra::Vector,
		radius: algebra::Scalar,
	}
}

impl Primitive {
	pub fn intersect(&self, ray: ray::Ray, min_d: algebra::Scalar, max_d: algebra::Scalar) -> std::option::Option<algebra::Vector> {
		match self {
			Primitive::Sphere { position, radius } => {
				let orig_to_center: algebra::Vector = ray.orig - *position;
				let a = ray.dir * ray.dir;
				let b = 2.0 * orig_to_center * ray.dir;
				let c = orig_to_center.norm_sqr() - radius * radius;
				let delta = b * b - 4.0 * a * c;
				if delta < 0.0 {
					std::option::Option::None
				} else {
					let mut t: algebra::Scalar = (-b - delta.sqrt()) / 2.0 / a;
					if t < min_d || t > max_d {
						t = (-b + delta.sqrt()) / 2.0 / a;
						if t < min_d || t > max_d {
							std::option::Option::None
						} else {
							std::option::Option::Some(ray.point_on_line(t))
						}
					} else {
						std::option::Option::Some(ray.point_on_line(t))
					}
				}
			}
		}
	}

	pub fn normal(&self, point: algebra::Vector) -> algebra::Vector {
		match self {
			Primitive::Sphere { position, radius } => {
				(point - *position).normalize()
			}
		}
	}
}
