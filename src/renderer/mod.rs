use crate::algebra;
use crate::camera;
use crate::output;
use crate::primitives;
use crate::ray;
use crate::scene;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Renderer {
    pub scene: scene::Scene,
    pub output: output::ImageFile,
    pub aa_samples: u32,
    pub chunk_size_exp: u32,
}

type RawPixel = (algebra::Scalar, algebra::Scalar, algebra::Scalar);

impl Renderer {
    pub fn render(&mut self) {
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
                self.output
                    .set_pixel(i, j, pix_grid[(j * self.output.width + i) as usize]);
            }
        }
    }

    fn trace(&self, x: u32, y: u32) -> (f64, f64, f64) {
        let camera: &camera::Camera = &self.scene.camera;
        let biggest_distance: algebra::Scalar = algebra::Scalar::MAX;
        let mut rng = thread_rng();
        let mut output_color: (f64, f64, f64) = (0.0, 0.0, 0.0);

        let mut closest_obj: std::option::Option<&primitives::Primitive>;
        let mut camera_plane_vector: algebra::Vector;
        let mut d: algebra::Scalar;
        let mut normal: algebra::Vector;
        let mut rand_x: algebra::Scalar;
        let mut rand_y: algebra::Scalar;

        for i in 0..self.aa_samples {
            closest_obj = std::option::Option::None;

            rand_x = rng.gen_range(-1.0..1.0);
            rand_y = rng.gen_range(-1.0..1.0);
            camera_plane_vector = (camera.ul_corner
                + ((x as f64) + rand_x) * camera.horizontal_step
                - ((y as f64) + rand_y) * camera.vertical_step)
                .normalize();
            d = biggest_distance;
            normal = algebra::Vector::new(0.0, 0.0, 0.0);
            let primary_ray =
                ray::Ray::new(algebra::Vector::new(0.0, 0.0, 0.0), camera_plane_vector);
            for obj in &self.scene.objects {
                match obj
                    .shape
                    .intersect(&primary_ray, camera.min_clip, camera.max_clip)
                {
                    std::option::Option::Some(point) => {
                        let normsq = (point - camera_plane_vector).norm_sqr();
                        if normsq < d {
                            d = normsq;
                            closest_obj = std::option::Option::Some(obj);
                            normal = obj.shape.normal(point);
                        }
                    }
                    std::option::Option::None => {
                        continue;
                    }
                }
            }

            match closest_obj {
                std::option::Option::None => {
                    output_color.0 += 0.0;
                    output_color.1 += 0.0;
                    output_color.2 += 0.0;
                }
                std::option::Option::Some(object) => {
                    //objects normal
                    output_color.0 += normal.x;
                    output_color.1 += normal.y;
                    output_color.2 += normal.z;
                }
            }
        }
        output_color.0 = output_color.0 / (self.aa_samples as f64);
        output_color.1 = output_color.1 / (self.aa_samples as f64);
        output_color.2 = output_color.2 / (self.aa_samples as f64);
        output_color
    }
}
