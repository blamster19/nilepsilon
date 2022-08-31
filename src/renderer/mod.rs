use crate::algebra;
use crate::camera;
use crate::constants;
use crate::materials;
use crate::output;
use crate::primitives;
use crate::ray;
use crate::sampler;
use crate::scene;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Renderer {
	pub scene: scene::Scene,
	pub output: output::ImageFile,
	pub aa_samples: usize,
	pub chunk_size_exp: u32,
	pub lights: Vec<usize>,
	pub max_depth: u32,
}

type RawPixel = (algebra::Scalar, algebra::Scalar, algebra::Scalar);

impl Renderer {
	pub fn render(&mut self) {
		// list all lights in the scene
		self.list_lights();

		let chunk_size = 2_u32.pow(self.chunk_size_exp).try_into().unwrap();
		let mut pix_grid =
			vec![(0.0, 0.0, 0.0); (self.output.width * self.output.height).try_into().unwrap()];
		pix_grid
			.par_chunks_mut(chunk_size)
			.enumerate()
			.for_each(|(chunk_index, chunk)| {
				for (pix_index, pix) in chunk.iter_mut().enumerate() {
					let offset = (chunk_index * chunk_size) as u32;
					let j = offset / self.output.width;
					let i = offset - j * self.output.height + pix_index as u32;
					*pix = self.trace(i, j);
				}
			});
		for i in 0..self.output.width {
			for j in 0..self.output.height {
				self.output.set_pixel(
					i,
					j,
					self.xyz_to_cie_rgb_d65(pix_grid[(j * self.output.width + i) as usize]),
				);
			}
		}
	}

	fn list_lights(&mut self) {
		for (index, obj) in self.scene.objects.iter().enumerate() {
			match obj.material.emitter {
				materials::EmissionType::NonEmissive => {
					continue;
				}
				_ => {
					self.lights.push(index);
				}
			}
		}
	}

	fn trace(&self, x: u32, y: u32) -> (f64, f64, f64) {
		let camera: &camera::Camera = &self.scene.camera;
		let mut sampler = sampler::Sampler { rng: thread_rng() };
		let mut output_color: (f64, f64, f64) = (0.0, 0.0, 0.0);

		let mut camera_plane_vector: algebra::Vector;
		let mut rand_x: algebra::Scalar;
		let mut rand_y: algebra::Scalar;
		let mut wavelength: algebra::Scalar;
		let mut wavelength_bunch: algebra::WavelengthBunch;
		let mut radiance = algebra::WavelengthBunch(0.0, 0.0, 0.0, 0.0);
		let mut temp_color: RawPixel;
		let camera_samples = sampler.random_list_2d(self.aa_samples, -1.0, 1.0);
		let wavelength_samples = sampler.random_list_1d(self.aa_samples, 360.0e-9, 650.0e-9);
		// source: https://jo.dreggn.org/home/2014_herowavelength.pdf
		let rot_func = |lambda, j| {
			((lambda * 1e9 - 360.0 + j / 4.0 * 290.0) as i32 % 290 + 360) as algebra::Scalar * 1e-9
		};

		for i in 0..self.aa_samples {
			// generate camera ray
			rand_x = camera_samples[i].0;
			rand_y = camera_samples[i].1;
			camera_plane_vector = (camera.ul_corner
				+ ((x as f64) + rand_x) * camera.horizontal_step
				- ((y as f64) + rand_y) * camera.vertical_step)
				.normalize();
			let primary_ray =
				ray::Ray::new(algebra::Vector::new(0.0, 0.0, 0.0), camera_plane_vector);

			// integrate
			wavelength = wavelength_samples[i];
			wavelength_bunch = algebra::WavelengthBunch(
				wavelength,
				rot_func(wavelength, 1.0),
				rot_func(wavelength, 2.0),
				rot_func(wavelength, 3.0),
			);
			radiance = self.integrate(primary_ray, self.max_depth, wavelength_bunch, &mut sampler);

			// compute color
			let tc0 = self.wavelength_to_xyz(wavelength_bunch.0);
			let tc1 = self.wavelength_to_xyz(wavelength_bunch.1);
			let tc2 = self.wavelength_to_xyz(wavelength_bunch.2);
			let tc3 = self.wavelength_to_xyz(wavelength_bunch.3);
			temp_color = (
				(tc0.0 * radiance.0 + tc1.0 * radiance.1 + tc2.0 * radiance.2 + tc3.0 * radiance.3)
					/ 4.0,
				(tc0.1 * radiance.0 + tc1.1 * radiance.1 + tc2.1 * radiance.2 + tc3.1 * radiance.3)
					/ 4.0,
				(tc0.2 * radiance.0 + tc1.2 * radiance.1 + tc2.2 * radiance.2 + tc3.2 * radiance.3)
					/ 4.0,
			);
			output_color.0 += temp_color.0;
			output_color.1 += temp_color.1;
			output_color.2 += temp_color.2;
		}
		output_color.0 /= self.aa_samples as f64;
		output_color.1 /= self.aa_samples as f64;
		output_color.2 /= self.aa_samples as f64;
		output_color
	}

	fn integrate(
		&self,
		ray: ray::Ray,
		depth: u32,
		wavelengths: algebra::WavelengthBunch,
		sampler: &mut sampler::Sampler,
	) -> algebra::WavelengthBunch {
		// find closest intersection
		let closest_obj: std::option::Option<&primitives::Primitive>;
		let intersection: algebra::Vector;
		let normal: algebra::Vector;
		(closest_obj, intersection, normal) =
			self.find_intersection(&ray, algebra::Scalar::EPSILON, algebra::Scalar::INFINITY);

		match closest_obj {
			std::option::Option::None => algebra::WavelengthBunch(
				self.scene
					.background
					.return_radiance(ray.dir, wavelengths.0),
				self.scene
					.background
					.return_radiance(ray.dir, wavelengths.1),
				self.scene
					.background
					.return_radiance(ray.dir, wavelengths.2),
				self.scene
					.background
					.return_radiance(ray.dir, wavelengths.3),
			),
			std::option::Option::Some(object) => {
				let mut radiance = algebra::WavelengthBunch(
					object.material.return_emission_radiance(wavelengths.0),
					object.material.return_emission_radiance(wavelengths.1),
					object.material.return_emission_radiance(wavelengths.2),
					object.material.return_emission_radiance(wavelengths.3),
				);
				if depth > 0 {
					let mut theta_o: algebra::Scalar = 0.0;
					let mut phi_o: algebra::Scalar = 0.0;
					let mut theta_i: algebra::Scalar = 0.0;
					let mut phi_i: algebra::Scalar = 0.0;
					let basis: algebra::Basis = object.material.new_basis(normal);

					// pick random direction
					let rand_rays: Vec<(f64, f64)> = sampler.random_list_2d(1, 0.0, 1.0);
					(theta_o, phi_o) =
						object
							.material
							.return_direction(theta_i, phi_i, rand_rays[0]);

					let mut next_ray: ray::Ray = ray::Ray::new(
						intersection,
						basis.basis_to_world(basis.spherical_to_basis(theta_o, phi_o)),
					);
					let mut contrib = self.integrate(next_ray, depth - 1, wavelengths, sampler);
					let mut surface_response = algebra::WavelengthBunch(
						object.material.return_scatter_radiance(
							theta_i,
							phi_i,
							theta_o,
							phi_o,
							wavelengths.0,
						),
						object.material.return_scatter_radiance(
							theta_i,
							phi_i,
							theta_o,
							phi_o,
							wavelengths.1,
						),
						object.material.return_scatter_radiance(
							theta_i,
							phi_i,
							theta_o,
							phi_o,
							wavelengths.2,
						),
						object.material.return_scatter_radiance(
							theta_i,
							phi_i,
							theta_o,
							phi_o,
							wavelengths.3,
						),
					);
					let mut sample_pdf_inv = algebra::WavelengthBunch(
						1.0 / object.material.return_pdf(
							next_ray.dir,
							ray.dir,
							normal,
							wavelengths.0,
						),
						1.0 / object.material.return_pdf(
							next_ray.dir,
							ray.dir,
							normal,
							wavelengths.1,
						),
						1.0 / object.material.return_pdf(
							next_ray.dir,
							ray.dir,
							normal,
							wavelengths.2,
						),
						1.0 / object.material.return_pdf(
							next_ray.dir,
							ray.dir,
							normal,
							wavelengths.3,
						),
					);
					let cos_theta_i = (normal * next_ray.dir).abs();
					let cti = algebra::WavelengthBunch(
						cos_theta_i,
						cos_theta_i,
						cos_theta_i,
						cos_theta_i,
					);
					contrib = contrib * surface_response * sample_pdf_inv * cti;
					radiance = radiance + contrib;
				}
				radiance
			}
		}
	}

	fn find_intersection(
		&self,
		ray: &ray::Ray,
		min: algebra::Scalar,
		max: algebra::Scalar,
	) -> (
		std::option::Option<&primitives::Primitive>,
		algebra::Vector,
		algebra::Vector,
	) {
		let mut d: algebra::Scalar = max;
		let mut closest_obj: std::option::Option<&primitives::Primitive> =
			std::option::Option::None;
		let mut intersection: algebra::Vector = algebra::Vector::new(0.0, 0.0, 0.0);
		let mut normal: algebra::Vector = algebra::Vector::new(0.0, 0.0, 0.0);
		for obj in &self.scene.objects {
			match obj.shape.intersect(&ray, min, max) {
				std::option::Option::Some(point) => {
					let norm = (point - ray.orig).norm();
					if norm < d && norm > min {
						d = norm;
						closest_obj = std::option::Option::Some(obj);
						intersection = point;
						normal = obj.shape.normal(point);
					} else {
						continue;
					}
				}
				std::option::Option::None => {
					continue;
				}
			}
		}
		(closest_obj, intersection, normal)
	}

	// the algorithm assumes wavelengths out of range are invisible, therefore black
	fn wavelength_to_xyz(&self, lambda: algebra::Scalar) -> RawPixel {
		let index = lambda * 1e9 - 360.0;
		if index < 0.0 || index > 470.0 {
			(0.0, 0.0, 0.0)
		} else {
			constants::CIE_XYZ_1931_COLOR_MATCH_2_DEG[index as usize]
		}
	}
	// source:
	// https://www.cs.rit.edu/~ncs/color/t_convert.html#RGB%20to%20XYZ%20&%20XYZ%20to%20RGB
	fn xyz_to_cie_rgb_d65(&self, xyz: RawPixel) -> RawPixel {
		(
			3.240479 * xyz.0 - 1.537150 * xyz.1 - 0.498535 * xyz.2,
			-0.969256 * xyz.0 + 1.875992 * xyz.1 + 0.041556 * xyz.2,
			0.055648 * xyz.0 - 0.204043 * xyz.1 + 1.057311 * xyz.2,
		)
	}
}
