use crate::primitives;
use crate::camera;

pub enum PrimitiveType {
	Sphere(primitives::Sphere),
}

pub enum BackgroundType {
	SolidColor((f64, f64, f64)),
}

pub struct Scene {
	objects: Vec<PrimitiveType>,
	camera: camera::Camera,
	background: BackgroundType,
}
