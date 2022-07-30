use crate::primitives;
use crate::camera;

pub enum BackgroundType {
	SolidColor((f64, f64, f64)),
}

pub struct Scene {
	pub objects: Vec<primitives::Primitive>,
	pub camera: camera::Camera,
	pub background: BackgroundType,
}
