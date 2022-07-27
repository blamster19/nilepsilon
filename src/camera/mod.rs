use crate::algebra;

pub enum Lens {
	Perspective,
}

pub struct Camera {
	pub lens_type: Lens,
	pub min_clip: algebra::Scalar,
	pub max_clip: algebra::Scalar,
	pub focal_distance: algebra::Scalar,

	pupil_radius: algebra::Scalar,
	focal_length: algebra::Scalar,

	pub sensor_width: algebra::Scalar,
	pub sensor_height: algebra::Scalar, 
	pub canvas_pix_width: u32,
	pub canvas_pix_height: u32,

	//upper left canvas corner position
	pub ul_corner: algebra::Vector,
	//horizontal and vertical distance between canvas pixels in physical space
	pub horizontal_step: algebra::Scalar,
	pub vertical_step: algebra::Scalar,
}

impl Camera {
	pub fn new(
		lens_type: Lens,
		min_clip: algebra::Scalar,
		max_clip: algebra::Scalar,
		focal_distance: algebra::Scalar,
		pupil_radius: algebra::Scalar,
		focal_length: algebra::Scalar,
		sensor_width: algebra::Scalar,
		sensor_height: algebra::Scalar,
		canvas_pix_width: u32,
		canvas_pix_height: u32,
		) -> Camera {
			if min_clip <= 0.0 {
				panic!("Camera property `min_clip` must be greater than or equal to  0.0, got {}", min_clip);
			}
			if max_clip <= 0.0 {
				panic!("Camera property `max_clip` must be greater than or equal to 0.0, got {}", min_clip);
			}
			if focal_distance <= 0.0 {
				panic!("Camera property `focal_distance` must be greater than or equal to 0.0, got {}", min_clip);
			}
			if pupil_radius <= 0.0 {
				panic!("Camera property `pupil_radius` must be greater than or equal 0.0, got {}", min_clip);
			}
			if focal_length <= 0.0 {
				panic!("Camera property `focal_length` must be greater than or equal to  0.0, got {}", min_clip);
			}
			if sensor_width < 0.0 {
				panic!("Camera property `sensor_width` must be greater than 0.0, got {}", min_clip);
			}
			if sensor_height < 0.0 {
				panic!("Camera property `sensor_height` must be greater than 0.0, got {}", min_clip);
			}
			let hstep: algebra::Scalar = sensor_width/(canvas_pix_width) as algebra::Scalar;
			let vstep: algebra::Scalar = sensor_height/(canvas_pix_height) as algebra::Scalar;
			Camera {
				lens_type,
				min_clip,
				max_clip,
				focal_distance,
				pupil_radius,
				focal_length,
				sensor_width,
				sensor_height,
				canvas_pix_width,
				canvas_pix_height,
				ul_corner: algebra::Vector::new(
					-(hstep * (canvas_pix_width) as algebra::Scalar)/2.0,
					focal_length,
					-(vstep * (canvas_pix_height) as algebra::Scalar)/2.0,
					),
				horizontal_step: hstep,
				vertical_step: vstep,
			}
		}
}
