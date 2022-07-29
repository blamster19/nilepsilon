use crate::primitives;
use crate::camera;

pub enum PrimitiveType {
	Sphere(primitives::Sphere),
}

pub enum BackgroundType {
	SolidColor((f64, f64, f64)),
}

pub struct Scene {
	pub objects: Vec<PrimitiveType>,
	pub camera: camera::Camera,
	pub background: BackgroundType,
}
