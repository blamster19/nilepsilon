use crate::primitives;
use crate::camera;
use crate::algebra;

pub enum BackgroundType {
	SolidColor((f64, f64, f64)),
}

pub struct Background {
	color: BackgroundType,
	radiance: algebra::Scalar,
}

impl Background {
	pub fn return_color(&self, dir: algebra::Vector) -> (f64, f64, f64) {
		match self.color {
			BackgroundType::SolidColor(color) => {
				color
			}
		}
	}
}

pub struct Scene {
	pub objects: Vec<primitives::Primitive>,
	pub camera: camera::Camera,
	pub background: Background,
}
