#[derive(Default, Clone)]
//in general values are between 0 and 1; might allow for future HDR output
struct Pixel {
	r: f64,
	g: f64,
	b: f64,
}

pub enum Format {
	PPM,
}

pub struct ImageFile {
	pub width: u32,
	pub height: u32,
	image: Vec<Pixel>,
}

impl ImageFile {
	pub fn new(width: u32, height: u32) -> ImageFile {
		ImageFile {
			width,
			height,
			image: vec![Pixel::default(); (width * height).try_into().unwrap()],
		}
	}

	pub fn set_pixel(&mut self, x: u32, y: u32, value: (f64, f64, f64)) {
		let index = (y * self.width + x) as usize;
		self.image[index].r = value.0;
		self.image[index].g = value.1;
		self.image[index].b = value.2;
	}

	pub fn out(&self, format: Format) {
		match format {
			Format::PPM => {
				println!("P3");
				println!("{} {}", self.width, self.height);
				println!("255");
				for pixel in &self.image {
					println!(
						"{} {} {}",
						((pixel.r * 255.0) as u8).clamp(0, 255),
						((pixel.g * 255.0) as u8).clamp(0, 255),
						((pixel.b * 255.0) as u8).clamp(0, 255)
					);
				}
			}
		}
	}
}
