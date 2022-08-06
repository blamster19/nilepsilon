use crate::primitives;
use crate::camera;
use crate::algebra;

pub enum BackgroundType {
	SolidColor((f64, f64, f64)),
}

impl BackgroundType {
	pub fn return_color(&self, dir: algebra::Vector) -> (f64, f64, f64) {
		match self {
			BackgroundType::SolidColor(color) => {
				*color
			}
		}
	}
}

pub struct Scene {
	pub objects: Vec<primitives::Primitive>,
	pub camera: camera::Camera,
	pub background: BackgroundType,
}
