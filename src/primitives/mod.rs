use crate::algebra;
use crate::ray;

#[derive(Clone, Copy)]
pub enum Primitive {
	Sphere {
		position: algebra::Vector,
		radius: algebra::Scalar,
	},
	Plane {
		position: algebra::Vector,
		normal: algebra::Vector,
	},
	Triangle {
		v1: algebra::Vector,
		v2: algebra::Vector,
		v3: algebra::Vector,

		v1v2: algebra::Vector,
		v1v3: algebra::Vector,
	},
}

impl Primitive {
	pub fn intersect(&self, ray: &ray::Ray, min_d: algebra::Scalar, max_d: algebra::Scalar) -> std::option::Option<algebra::Vector> {
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

			Primitive::Plane { position, normal } => {
				let divisor: algebra::Scalar = (*normal) * ray.dir;
				if divisor == 0.0 {
					std::option::Option::None
				} else {
					let t: algebra::Scalar = ((*position - ray.orig) * (*normal)) / divisor;
					if t < min_d || t > max_d {
						std::option::Option::None
					} else {
						std::option::Option::Some(ray.point_on_line(t))
					}
				}
			}

			Primitive::Triangle { v1, v2, v3, v1v2, v1v3, } => {
				let point: algebra::Vector = ray.dir % (*v1v3);
				let mut det: algebra::Scalar = (*v1v2) * point;
				if det.abs() < algebra::Scalar::EPSILON {
					std::option::Option::None
				} else {
					det = 1.0 / det;
					let tv: algebra::Vector = ray.orig - *v1;
					let u: algebra::Scalar = tv * point * det;
					if u < 0.0 || u > 1.0 {
						return std::option::Option::None;
					}
					let qv: algebra::Vector = tv % (*v1v2);
					let v: algebra::Scalar = ray.dir * qv * det;
					if v < 0.0 || u + v > 1.0 {
						return std::option::Option::None;
					}
					let t: algebra::Scalar = (*v1v2) * qv * det;
					std::option::Option::Some(*v1 % *v2)
				}
			}
		}
	}

	pub fn normal(&self, point: algebra::Vector) -> algebra::Vector {
		match self {
			Primitive::Sphere { position, radius } => {
				(point - *position).normalize()
			}

			Primitive::Plane { position, normal } => {
				(point - *normal).normalize()
			}

			Primitive::Triangle { v1, v2, v3, .. } => {
				((*v2 - *v1) % (*v3 - *v1)).normalize()
			}
		}
	}

	pub fn new_sphere(position: algebra::Vector, radius: algebra::Scalar) -> Primitive {
		Primitive::Sphere {
			position,
			radius,
		}
	}

	pub fn new_plane(position: algebra::Vector, normal: algebra::Vector) -> Primitive {
		Primitive::Plane {
			position,
			normal,
		}
	}

	pub fn new_triangle(v1: algebra::Vector, v2: algebra::Vector, v3: algebra::Vector) -> Primitive {
		let v1v2: algebra::Vector = v2 - v1;
		let v1v3: algebra::Vector = v3 - v1;

		Primitive::Triangle {
			v1,
			v2,
			v3,
			v1v2: v1v2,
			v1v3: v1v3,
		}
	}
}
