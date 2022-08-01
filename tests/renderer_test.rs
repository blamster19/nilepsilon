use nilepsilon::algebra;
use nilepsilon::camera;
use nilepsilon::primitives;
use nilepsilon::scene;
use nilepsilon::renderer;
use nilepsilon::output;

#[test]
fn renderer_three_spheres_raster() {
	let cam = camera::Camera::new(
		camera::Lens::Perspective,
		0.035,
		0.064,
		0.048,
		640,
		480,
		0.0,
		1000.0,
		);
	let sph1 = primitives::Primitive::Sphere {position: algebra::Vector::new(0.0, 2.0, 0.0), radius: 1.0};
	let sph2 = primitives::Primitive::Sphere {position: algebra::Vector::new(-2.0, 2.0, 0.0), radius: 1.0};
	let sph3 = primitives::Primitive::Sphere {position: algebra::Vector::new(2.0, 2.0, 0.0), radius: 1.0};
	let sc = scene::Scene {
		objects:vec![sph1, sph2, sph3],
		camera: cam,
		background: scene::BackgroundType::SolidColor((0.5, 0.5, 0.5)),
		};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 10,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}

#[test]
fn renderer_overlapping_spheres_raster() {
	let cam = camera::Camera::new(
		camera::Lens::Perspective,
		0.035,
		0.064,
		0.048,
		640,
		480,
		0.0,
		1000.0,
		);
	let sph1 = primitives::Primitive::Sphere {position: algebra::Vector::new(0.0, 2.0, 0.0), radius: 1.0};
	let sph2 = primitives::Primitive::Sphere {position: algebra::Vector::new(-0.5, 4.0, 2.0), radius: 1.0};
	let sph3 = primitives::Primitive::Sphere {position: algebra::Vector::new(0.5, 6.0, 3.0), radius: 1.0};
	let sc = scene::Scene {
		objects:vec![sph1, sph2, sph3],
		camera: cam,
		background: scene::BackgroundType::SolidColor((0.5, 0.5, 0.5)),
		};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 10,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}

#[test]
fn renderer_sphere_plane_raster() {
	let cam = camera::Camera::new(
		camera::Lens::Perspective,
		0.035,
		0.064,
		0.048,
		640,
		480,
		0.0,
		1000.0,
		);
	let sph1 = primitives::Primitive::Sphere {position: algebra::Vector::new(0.0, 2.0, 0.0), radius: 1.0};
	let pln1 = primitives::Primitive::Plane {position: algebra::Vector::new(0.0, 0.0, -0.5), normal: algebra::Vector::new(0.0, 0.0, 1.0)};
	let sc = scene::Scene {
		objects:vec![sph1, pln1],
		camera: cam,
		background: scene::BackgroundType::SolidColor((0.5, 0.5, 0.5)),
		};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 64,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}

#[test]
fn renderer_triangle_algorithm_speedtest() {
	let cam = camera::Camera::new(
		camera::Lens::Perspective,
		0.035,
		0.064,
		0.048,
		640,
		480,
		0.0,
		1000.0,
		);
	let tri = primitives::Primitive::new_triangle(algebra::Vector::new(-1.0, 1.0, -1.0), algebra::Vector::new(-2.0, 1.0, 1.0), algebra::Vector::new(1.0, 1.0,0.0));

	let sc = scene::Scene {
		objects:vec![tri],
		camera: cam,
		background: scene::BackgroundType::SolidColor((0.5, 0.5, 0.5)),
		};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 1,
	};
	for _i in 0..200 {
		dis.render();
	}
//	dis.output.out(output::Format::PPM);
}
