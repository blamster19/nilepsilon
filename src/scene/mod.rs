use crate::primitives;
use crate::camera;

pub enum Primitive_type {
	Sphere(primitives::Sphere),
}

pub enum Background_type {
	solid_color((f64, f64, f64)),
}

pub struct Scene {
	objects: Vec<Primitive_type>,
	camera: camera::Camera,
	background: Background_type,
}
