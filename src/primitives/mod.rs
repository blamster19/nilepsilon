use crate::algebra;
use crate::ray;
use crate::materials;

pub struct Primitive {
	pub shape: Shape,
	pub material: materials::Material,
}

impl Primitive {
	pub fn new_sphere(position: algebra::Vector, radius: algebra::Scalar, material: materials::Material) -> Primitive {
		Primitive {
			material,
			shape: Shape::Sphere {
				position,
				radius,
			},
		}
	}

	pub fn new_plane(position: algebra::Vector, normal: algebra::Vector, material: materials::Material) -> Primitive {
		Primitive {
			material,
			shape: Shape::Plane {
				position,
				normal: normal.normalize(),
			},
		}
	}

	pub fn new_triangle(v1: algebra::Vector, v2: algebra::Vector, v3: algebra::Vector, material: materials::Material) -> Primitive {
		let v1v2: algebra::Vector = v2 - v1;
		let v1v3: algebra::Vector = v3 - v1;

		Primitive {
			material,
			shape: Shape::Triangle {
				v1,
				v2,
				v3,
				v1v2: v1v2,
				v1v3: v1v3,
			},
		}
	}
}

#[derive(Clone, Copy)]
pub enum Shape {
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

impl Shape {
	pub fn intersect(&self, ray: &ray::Ray, min_d: algebra::Scalar, max_d: algebra::Scalar) -> std::option::Option<algebra::Vector> {
		match self {
			Shape::Sphere { position, radius, .. } => {
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

			Shape::Plane { position, normal, .. } => {
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

			Shape::Triangle { v1, v2, v3, v1v2, v1v3, .. } => {
				let point: algebra::Vector = ray.dir % (*v1v3);
				let mut det: algebra::Scalar = (*v1v2) * point;
				if det.abs() < algebra::Scalar::EPSILON {
					std::option::Option::None
				} else {
					det = 1.0 / det;
					let tv: algebra::Vector = ray.orig - *v1;
					let v: algebra::Scalar = tv * point * det;
					if v < 0.0 || v > 1.0 {
						return std::option::Option::None;
					}
					let qv: algebra::Vector = tv % (*v1v2);
					let w: algebra::Scalar = ray.dir * qv * det;
					if w < 0.0 || v + w > 1.0 {
						return std::option::Option::None;
					}
					let u: algebra::Scalar = (*v1v2) * qv * det;
					std::option::Option::Some(u * (*v1) + v * (*v2) + w * (*v3))
				}
			}
		}
	}

	pub fn normal(&self, point: algebra::Vector) -> algebra::Vector {
		match self {
			Shape::Sphere { position, radius, .. } => {
				(point - *position).normalize()
			}

			Shape::Plane { position, normal, .. } => {
				*normal
			}

			Shape::Triangle { v1, v2, v3, .. } => {
				((*v2 - *v1) % (*v3 - *v1)).normalize()
			}
		}
	}
}
