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

			Primitive::Triangle { v1, v2, v3 } => {
				//does ray intersect the triangle's plane at all
				let a: algebra::Vector = *v2 - *v1;
				let b: algebra::Vector = *v3 - *v1;
				let normal: algebra::Vector = (a % b).normalize();

				let tri_plane = Primitive::Plane { position: *v1, normal: normal };
				match tri_plane.intersect(ray, min_d, max_d) {
					std::option::Option::None => {
						std::option::Option::None
					}
					std::option::Option::Some(point) => {
						//convert to barycentric
						let c: algebra::Vector = point - *v1;
						let d_aa: algebra::Scalar = a.norm_sqr();
						let d_ab: algebra::Scalar = a * b;
						let d_bb: algebra::Scalar= b.norm_sqr();
						let d_ca: algebra::Scalar = c * a;
						let d_cb: algebra::Scalar = c * b;
						let denom: algebra::Scalar = d_aa * d_bb - d_ab * d_ab;

						let v: algebra::Scalar = (d_bb * d_ca - d_ab * d_cb) / denom;
						let w: algebra::Scalar = (d_aa * d_cb - d_ab * d_ca) / denom;
						let u: algebra::Scalar = 1.0 - v - w;

						if u < 0.0 || v < 0.0 || w < 0.0 {
							std::option::Option::None
						} else {
							std::option::Option::Some(point)
						}
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

			Primitive::Plane { position, normal } => {
				(point - *normal).normalize()
			}

			Primitive::Triangle { v1, v2, v3 } => {
				((*v2 - *v1) % (*v3 - *v1)).normalize()
			}
		}
	}
}
