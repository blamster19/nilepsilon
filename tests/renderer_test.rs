use nilepsilon::algebra;
use nilepsilon::camera;
use nilepsilon::materials;
use nilepsilon::output;
use nilepsilon::primitives;
use nilepsilon::renderer;
use nilepsilon::scene;

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
	let bg = scene::Background {
		color: scene::BackgroundType::BlackBody(2500.0),
		radiance: 10000.0,
	};
	let diff1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.6],
	);
	let sph1 =
		primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 2.0, 0.0), 1.0, diff1.clone());
	let sph2 =
		primitives::Primitive::new_sphere(algebra::Vector::new(-2.0, 2.0, 0.0), 1.0, diff1.clone());
	let sph3 =
		primitives::Primitive::new_sphere(algebra::Vector::new(2.0, 2.0, 0.0), 1.0, diff1.clone());
	let sc = scene::Scene {
		objects: vec![sph1, sph2, sph3],
		camera: cam,
		background: bg,
	};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 10,
		chunk_size_exp: 16,
		lights: vec![],
		max_depth: 5,
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
	let bg = scene::Background {
		color: scene::BackgroundType::BlackBody(2500.0),
		radiance: 10000.0,
	};
	let diff1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.6],
	);
	let sph1 =
		primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 2.0, 0.0), 1.0, diff1.clone());
	let sph2 =
		primitives::Primitive::new_sphere(algebra::Vector::new(-0.5, 4.0, 2.0), 1.0, diff1.clone());
	let sph3 =
		primitives::Primitive::new_sphere(algebra::Vector::new(0.5, 6.0, 3.0), 1.0, diff1.clone());
	let sc = scene::Scene {
		objects: vec![sph1, sph2, sph3],
		camera: cam,
		background: bg,
	};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 10,
		chunk_size_exp: 16,
		lights: vec![],
		max_depth: 5,
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
	let bg = scene::Background {
		color: scene::BackgroundType::BlackBody(2500.0),
		radiance: 10000.0,
	};
	let diff1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.6],
	);
	let sph1 =
		primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 2.0, 0.0), 1.0, diff1.clone());
	let pln1 = primitives::Primitive::new_plane(
		algebra::Vector::new(0.0, 0.0, -0.5),
		algebra::Vector::new(0.0, 0.0, 1.0),
		diff1.clone(),
	);
	let sc = scene::Scene {
		objects: vec![sph1, pln1],
		camera: cam,
		background: bg,
	};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 64,
		chunk_size_exp: 16,
		lights: vec![],
		max_depth: 5,
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
	let bg = scene::Background {
		color: scene::BackgroundType::BlackBody(2500.0),
		radiance: 10000.0,
	};
	let diff1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.6],
	);
	let tri = primitives::Primitive::new_triangle(
		algebra::Vector::new(-1.0, 1.0, -1.0),
		algebra::Vector::new(-2.0, 1.0, 1.0),
		algebra::Vector::new(1.0, 1.0, 0.0),
		diff1,
	);

	let sc = scene::Scene {
		objects: vec![tri],
		camera: cam,
		background: bg,
	};
	let img = output::ImageFile::new(640, 480);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 1,
		chunk_size_exp: 16,
		lights: vec![],
		max_depth: 5,
	};
	for _i in 0..200 {
		dis.render();
	}
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
	let bg = scene::Background {
		color: scene::BackgroundType::BlackBody(2500.0),
		radiance: 10000.0,
	};
	let diffuse1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.6],
	);
	let back = primitives::Primitive::new_plane(
		algebra::Vector::new(0.0, 8.0, 0.0),
		algebra::Vector::new(0.0, -1.0, 0.0),
		diffuse1.clone(),
	);
	let right = primitives::Primitive::new_plane(
		algebra::Vector::new(2.0, 6.0, 0.0),
		algebra::Vector::new(-1.0, 0.0, 0.0),
		diffuse1.clone(),
	);
	let left = primitives::Primitive::new_plane(
		algebra::Vector::new(-2.0, 6.0, 0.0),
		algebra::Vector::new(1.0, 0.0, 0.0),
		diffuse1.clone(),
	);
	let ceil = primitives::Primitive::new_plane(
		algebra::Vector::new(0.0, 6.0, 2.0),
		algebra::Vector::new(0.0, 0.0, -1.0),
		diffuse1.clone(),
	);
	let floor = primitives::Primitive::new_plane(
		algebra::Vector::new(0.0, 6.0, -2.0),
		algebra::Vector::new(0.0, 0.0, 1.0),
		diffuse1.clone(),
	);
	let ball = primitives::Primitive::new_sphere(
		algebra::Vector::new(0.0, 5.0, -1.0),
		1.0,
		diffuse1.clone(),
	);

	let sc = scene::Scene {
		objects: vec![back, right, left, ceil, floor, ball],
		camera: cam,
		background: bg,
	};
	let img = output::ImageFile::new(512, 512);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 10,
		chunk_size_exp: 4,
		lights: vec![],
		max_depth: 5,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}

#[test]
fn renderer_empty() {
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

	let bg = scene::Background {
		color: scene::BackgroundType::BlackBody(2500.0),
		radiance: 10000.0,
	};
	let diff1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.8],
	);
	let sc = scene::Scene {
		camera: cam,
		background: bg,
		objects: vec![],
	};
	let img = output::ImageFile::new(512, 512);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 100,
		chunk_size_exp: 4,
		lights: vec![],
		max_depth: 5,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}

#[test]
fn renderer_cornell_2() {
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

	let bg = scene::Background {
		color: scene::BackgroundType::BlackBodyNormalized(7000.0),
		radiance: 0.0,
	};
	let diff1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.8],
	);
	let red1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![-179.2, 6.0e8, -5e14],
	);
	let blue1 = materials::Material::new(
		materials::EmissionType::NonEmissive,
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![-99.2, 4.0e8, -4e14],
	);
	let lamp_one = materials::Material::new(
		materials::EmissionType::Cool {
			temperature: 6500.0,
			power: 100.0,
		},
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.8],
	);
	let lamp_two = materials::Material::new(
		materials::EmissionType::Cool {
			temperature: 5500.0,
			power: 66.0,
		},
		materials::SurfaceType::Dielectric { sigma: 0.6 },
		vec![0.8],
	);
	let ball1 =
		primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 5.0, -1.0), 1.0, diff1.clone());
	let ball2 =
		primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 5.0, 1.0), 1.0, diff1.clone());
	let floor1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(3.0, 8.0, -2.0),
		algebra::Vector::new(-3.0, 8.0, -2.0),
		algebra::Vector::new(-3.0, 0.0, -2.0),
		diff1.clone(),
	);
	let floor2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, 0.0, -2.0),
		algebra::Vector::new(3.0, 0.0, -2.0),
		algebra::Vector::new(3.0, 8.0, -2.0),
		diff1.clone(),
	);
	let left1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, 0.0, -2.0),
		algebra::Vector::new(-3.0, 8.0, -2.0),
		algebra::Vector::new(-3.0, 0.0, 4.0),
		red1.clone(),
	);
	let left2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, 8.0, 4.0),
		algebra::Vector::new(-3.0, 0.0, 4.0),
		algebra::Vector::new(-3.0, 8.0, -2.0),
		red1.clone(),
	);
	let right1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(3.0, 8.0, -2.0),
		algebra::Vector::new(3.0, 0.0, -2.0),
		algebra::Vector::new(3.0, 0.0, 4.0),
		blue1.clone(),
	);
	let right2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(3.0, 0.0, 4.0),
		algebra::Vector::new(3.0, 8.0, 4.0),
		algebra::Vector::new(3.0, 8.0, -2.0),
		blue1.clone(),
	);
	let ceil1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(3.0, 8.0, 4.0),
		algebra::Vector::new(-3.0, 0.0, 4.0),
		algebra::Vector::new(-3.0, 8.0, 4.0),
		diff1.clone(),
	);
	let ceil2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, 0.0, 4.0),
		algebra::Vector::new(3.0, 8.0, 4.0),
		algebra::Vector::new(3.0, 0.0, 4.0),
		diff1.clone(),
	);
	let back1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, 8.0, 4.0),
		algebra::Vector::new(3.0, 8.0, -2.0),
		algebra::Vector::new(3.0, 8.0, 4.0),
		diff1.clone(),
	);
	let back2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, 8.0, 4.0),
		algebra::Vector::new(-3.0, 8.0, -2.0),
		algebra::Vector::new(3.0, 8.0, -2.0),
		diff1.clone(),
	);
	let lamp1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(0.75, 6.0, 3.9),
		algebra::Vector::new(-0.75, 5.0, 3.9),
		algebra::Vector::new(-0.75, 6.0, 3.9),
		lamp_one.clone(),
	);
	let lamp2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-0.75, 5.0, 3.9),
		algebra::Vector::new(0.75, 6.0, 3.9),
		algebra::Vector::new(0.75, 5.0, 3.9),
		lamp_two.clone(),
	);
	let front1 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, -0.5, 4.0),
		algebra::Vector::new(3.0, -0.5, 4.0),
		algebra::Vector::new(3.0, -0.5, -2.0),
		diff1.clone(),
	);
	let front2 = primitives::Primitive::new_triangle(
		algebra::Vector::new(-3.0, -0.5, 4.0),
		algebra::Vector::new(3.0, -0.5, -2.0),
		algebra::Vector::new(-3.0, -0.5, -2.0),
		diff1.clone(),
	);
	let mut sc = scene::Scene {
		camera: cam,
		background: bg,
		objects: vec![
			ball1, floor1, floor2, left1, left2, right1, right2, ceil1, ceil2, back1, back2, lamp1,
			lamp2,
		],
	};
	let img = output::ImageFile::new(512, 512);
	let mut dis = renderer::Renderer {
		scene: sc,
		output: img,
		aa_samples: 200,
		chunk_size_exp: 6,
		lights: vec![],
		max_depth: 5,
	};
	dis.render();
	dis.output.out(output::Format::PPM);
}
