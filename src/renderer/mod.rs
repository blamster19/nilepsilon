use crate::algebra;
use crate::ray;
use crate::camera;
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
		//temporary rendering: black - no hit, white - hit
		let mut c: (f64, f64, f64) = (0.0, 0.0, 0.0);
		for obj in &self.scene.objects {
			let camera_plane_vector: algebra::Vector = (camera.ul_corner + (x as f64) * camera.horizontal_step - (y as f64) * camera.vertical_step).normalize();
			if obj.intersect(ray::Ray::new(algebra::Vector::new(0.0, 0.0, 0.0), camera_plane_vector)) {
				c = (1.0, 1.0, 1.0);
			}
		}
		c
	}
}
