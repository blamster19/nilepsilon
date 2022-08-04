use nilepsilon::algebra;
use nilepsilon::camera;
use nilepsilon::primitives;
use nilepsilon::materials;
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
	let diff1 = materials::Material {};
	let sph1 = primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 2.0, 0.0), 1.0, diff1.clone());
	let sph2 = primitives::Primitive::new_sphere(algebra::Vector::new(-2.0, 2.0, 0.0),  1.0, diff1.clone());
	let sph3 = primitives::Primitive::new_sphere(algebra::Vector::new(2.0, 2.0, 0.0), 1.0, diff1.clone());
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
	let diff1 = materials::Material {};
	let sph1 = primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 2.0, 0.0), 1.0, diff1.clone());
	let sph2 = primitives::Primitive::new_sphere(algebra::Vector::new(-0.5, 4.0, 2.0), 1.0, diff1.clone());
	let sph3 = primitives::Primitive::new_sphere(algebra::Vector::new(0.5, 6.0, 3.0), 1.0, diff1.clone());
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
	let diff1 = materials::Material {};
	let sph1 = primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 2.0, 0.0), 1.0, diff1.clone());
	let pln1 = primitives::Primitive::new_plane( algebra::Vector::new(0.0, 0.0, -0.5), algebra::Vector::new(0.0, 0.0, 1.0), diff1.clone());
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
	let diff1 = materials::Material {};
	let tri = primitives::Primitive::new_triangle(algebra::Vector::new(-1.0, 1.0, -1.0), algebra::Vector::new(-2.0, 1.0, 1.0), algebra::Vector::new(1.0, 1.0,0.0), diff1);

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

#[test]
fn renderer_cornell_1() {
	let cam = camera::Camera::new(
		camera::Lens::Perspective,
		0.035,
		0.0512,
		0.0512,
		512,
		512,
		0.0,
		1000.0,
		);
	let diffuse1 = materials::Material {};
	let back = primitives::Primitive::new_plane(algebra::Vector::new(0.0, 8.0, 0.0), algebra::Vector::new(0.0, -1.0, 0.0), diffuse1.clone());
	let right = primitives::Primitive::new_plane(algebra::Vector::new(2.0, 6.0, 0.0), algebra::Vector::new(-1.0, 0.0, 0.0), diffuse1.clone());
	let left = primitives::Primitive::new_plane(algebra::Vector::new(-2.0, 6.0, 0.0), algebra::Vector::new(1.0, 0.0, 0.0), diffuse1.clone());
	let ceil = primitives::Primitive::new_plane(algebra::Vector::new(0.0, 6.0, 2.0), algebra::Vector::new(0.0, 0.0, -1.0), diffuse1.clone());
	let floor = primitives::Primitive::new_plane(algebra::Vector::new(0.0, 6.0, -2.0), algebra::Vector::new(0.0, 0.0, 1.0), diffuse1.clone());
	let ball = primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 5.0, -1.0), 1.0, diffuse1.clone());

	let sc = scene::Scene {
		objects:vec![back, right, left, ceil, floor, ball],
		camera: cam,
		background: scene::BackgroundType::SolidColor((0.5, 0.5, 0.5)),
		};
	let img = output::ImageFile::new(512, 512);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 10,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}
