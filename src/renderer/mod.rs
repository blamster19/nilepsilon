use crate::algebra;
use crate::ray;
use crate::camera;
use crate::primitives;
use crate::output;
use crate::scene;

pub struct Renderer {
	pub scene: scene::Scene,
	pub output: output::ImageFile,
}

impl Renderer {
	pub fn render(&mut self) {
		for i in 0..self.output.width {
			for j in 0..self.output.height {
				self.output.set_pixel(i, j, self.trace(i, j));
			}
		}
	}

	fn trace(&self, x: u32, y: u32) -> (f64, f64, f64) {
		let camera: &camera::Camera = &self.scene.camera;
		let mut biggest_distance: algebra::Scalar = algebra::Scalar::MAX;
		let mut closest_obj: std::option::Option<&primitives::Primitive> = std::option::Option::None;
		let camera_plane_vector: algebra::Vector = (camera.ul_corner + (x as f64) * camera.horizontal_step - (y as f64) * camera.vertical_step).normalize();
		let mut d: algebra::Scalar = biggest_distance;
		let mut normal: algebra::Vector = algebra::Vector::new(0.0, 0.0, 0.0);

		let primary_ray = ray::Ray::new(algebra::Vector::new(0.0, 0.0, 0.0), camera_plane_vector);
		for obj in &self.scene.objects {
			match obj.intersect(&primary_ray, camera.min_clip, camera.max_clip) {
				std::option::Option::Some(point) => {
					let normsq = (point - camera_plane_vector).norm_sqr();
					if normsq < d {
						d = normsq;
						closest_obj = std::option::Option::Some(obj);
						normal = obj.normal(point);
					}
				}
				std::option::Option::None => {
					continue;
				}
			}
		}

		match closest_obj {
			std::option::Option::None => {
				(0.0, 0.0, 0.0)
			}
			std::option::Option::Some(object) => {
				//objects normal
				(normal.x, normal.y, normal.z)
			}
		}
	}
}
