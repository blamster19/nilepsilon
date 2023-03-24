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
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
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
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
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
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
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
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
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
        color: scene::BackgroundType::BlackBodyNormalized(2500.0),
        radiance: 10.0,
    };
    let diffuse1 = materials::Material::new(
        materials::EmissionType::NonEmissive,
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
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
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
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
fn renderer_furnace() {
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
        color: scene::BackgroundType::BlackBodyNormalized(6504.51),
        radiance: 1.0,
    };

    let specular = materials::Material::new(
        materials::EmissionType::NonEmissive,
        materials::SurfaceType::DielectricOpaque {
            //color: vec![-99.2, 4.0e8, -4e14],
            color: vec![0.5],
            roughness: 0.001,
        },
        1.5,
        0.0,
    );

    let ball1 = primitives::Primitive::new_sphere(
        algebra::Vector::new(0.0, 2.0, 0.0),
        1.0,
        specular.clone(),
    );

    let sc = scene::Scene {
        camera: cam,
        background: bg,
        objects: vec![ball1],
    };
    let img = output::ImageFile::new(512, 512);
    let mut dis = renderer::Renderer {
        scene: sc,
        output: img,
        aa_samples: 500,
        chunk_size_exp: 4,
        lights: vec![],
        max_depth: 2,
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
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.6],
            roughness: 0.8,
        },
        1.5,
        0.0,
    );
    let specular = materials::Material::new(
        materials::EmissionType::Fresnel,
        materials::SurfaceType::DielectricOpaque {
            //color: vec![-99.2, 4.0e8, -4e14],
            //color: vec![1.90511e-17, 2.97674e-9, 0.22952, 1.16378e7, 4.36303e14, 1.28923e22],
            color: vec![0.0],
            roughness: 0.01,
        },
        1.5,
        0.0,
    );
    //let glass = materials::Material::new(
    //    materials::EmissionType::NonEmissive,
        //	materials::EmissionType::Cool {
        //		temperature: 3000.0,
        //		power: 1.0,
        //	},
    //    materials::SurfaceType::DielectricTransparent { roughness: 0.1 },
    //    1.1,
    //    0.0,
    //);
    let metal = materials::Material::new(
        materials::EmissionType::NonEmissive,
        materials::SurfaceType::Conductor { roughness: 0.1 },
        0.05,
        3.9,
    );
    let red1 = materials::Material::new(
        materials::EmissionType::NonEmissive,
        materials::SurfaceType::DielectricOpaque {
            color: vec![-179.2, 6.0e8, -5e14],
            roughness: 0.8,
        },
        1.5,
        0.0,
    );
    let blue1 = materials::Material::new(
        materials::EmissionType::NonEmissive,
        materials::SurfaceType::DielectricOpaque {
            color: vec![-99.2, 4.0e8, -4e14],
            roughness: 0.8,
        },
        1.5,
        0.0,
    );
    let lamp_one = materials::Material::new(
        materials::EmissionType::Cool {
            temperature: 6500.0,
            power: 200.0,
        },
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.8],
            roughness: 0.8,
        },
        1.5,
        0.0,
    );
    let lamp_two = materials::Material::new(
        materials::EmissionType::Cool {
            temperature: 5500.0,
            power: 1.0,
        },
        materials::SurfaceType::DielectricOpaque {
            color: vec![0.8],
            roughness: 0.8,
        },
        1.5,
        0.0,
    );
    let balls_z = 0.0;
    let ball1 = primitives::Primitive::new_sphere(
        algebra::Vector::new(0.0, 7.0, -1.0 + balls_z),
        1.0,
        //specular.clone(),
        red1.clone(),
    );
    let ball3 =
        primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 6.5, 0.0), 1.0, red1.clone());
    let ball4 =
        primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 7.0, 3.0), 1.0, diff1.clone());
    let ball5 =
        primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 7.0, 1.0), 1.0, diff1.clone());
    let ball2 = primitives::Primitive::new_sphere(
        algebra::Vector::new(1.5, 5.0, -1.0 + balls_z),
        1.0,
        metal.clone(),
    );
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
        // !!!
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
    let lampz = 0.0;
    let lamp1 = primitives::Primitive::new_triangle(
        algebra::Vector::new(0.75, 6.0, 3.9 + lampz),
        algebra::Vector::new(-0.75, 6.0, 3.9 + lampz),
        algebra::Vector::new(-0.75, 5.0, 3.9 + lampz),
        lamp_one.clone(),
    );
    let lamp2 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-0.75, 5.0, 3.9 + lampz),
        algebra::Vector::new(0.75, 5.0, 3.9 + lampz),
        algebra::Vector::new(0.75, 6.0, 3.9 + lampz),
        lamp_one.clone(),
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
    //primitives::Primitive::new_sphere(algebra::Vector::new(0.0, 5.0, -1.0), 1.0, diff1.clone());
    let x_cube = -0.5;
    let y_cube = 2.0;
    let z_cube = 0.0; // 1.975;
    let cube_front1 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-1.0 + x_cube, 4.0 + y_cube, -2.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 4.0 + y_cube, -2.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 4.0 + y_cube, -1.0 + z_cube),
        diff1.clone(),
    );
    let cube_front2 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-1.0 + x_cube, 4.0 + y_cube, -2.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 4.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(-1.0 + x_cube, 4.0 + y_cube, -1.0 + z_cube),
        diff1.clone(),
    );
    let cube_up1 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-1.0 + x_cube, 4.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 4.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 6.0 + y_cube, -1.0 + z_cube),
        diff1.clone(),
    );
    let cube_up2 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-1.0 + x_cube, 4.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 6.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(-1.0 + x_cube, 6.0 + y_cube, -1.0 + z_cube),
        diff1.clone(),
    );
    let cube_back1 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-1.0 + x_cube, 6.0 + y_cube, -2.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 6.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 6.0 + y_cube, -2.0 + z_cube),
        diff1.clone(),
    );
    let cube_back2 = primitives::Primitive::new_triangle(
        algebra::Vector::new(-1.0 + x_cube, 6.0 + y_cube, -2.0 + z_cube),
        algebra::Vector::new(-1.0 + x_cube, 6.0 + y_cube, -1.0 + z_cube),
        algebra::Vector::new(1.0 + x_cube, 6.0 + y_cube, -1.0 + z_cube),
        diff1.clone(),
    );
    let mut sc = scene::Scene {
        camera: cam,
        background: bg,
        objects: vec![
            ball1, ball2, 	//	ball3,
            //			ball3, ball4, ball5,
            floor1, floor2, left1, left2, right1, right2, ceil1, ceil2, back1, back2, lamp1,
            lamp2,
            //		cube_front1, cube_front2, cube_up1, cube_up2, cube_back1, cube_back2
        ],
    };
    let img = output::ImageFile::new(512, 512);
    let mut dis = renderer::Renderer {
        scene: sc,
        output: img,
        aa_samples: 500,
        chunk_size_exp: 2,
        lights: vec![],
        max_depth: 3,
    };
    dis.render();
    dis.output.out(output::Format::PPM);
}
