use nilepsilon::output;

#[test]
fn output_gradient() {
	let mut img = output::ImageFile::new(256,256);
	for i in (0..256).rev() {
		for j in 0..256 {
			img.set_pixel(i, j, ((i as f64)/255.0, (j as f64)/255.0, 0.25));
		}
	}
	img.out(output::Format::PPM);
}

#[test]
fn output_quarter_circle() {
	let mut img = output::ImageFile::new(256,256);
	for i in (0..256).rev() {
		for j in 0..256 {
			let col = ((i*i+j*j < 256*256) as i32) as f64;
			img.set_pixel(i, j, (col, col, col));
		}
	}
	img.out(output::Format::PPM);
}

#[test]
fn output_color_wheels() {
	let mut img = output::ImageFile::new(256,256);
	let red = (85, 85);
	let green = (170, 85);
	let blue = (120, 158);
	let rad: isize = 60;
	for i in (0..256).rev() {
		for j in 0..256 {
			let r = ((((j as isize)-red.0).pow(2)+((i as isize)-red.1).pow(2) < rad.pow(2)) as i32) as f64;
			let g = ((((j as isize)-green.0).pow(2)+((i as isize)-green.1).pow(2) < rad.pow(2)) as i32) as f64;
			let b = ((((j as isize)-blue.0).pow(2)+((i as isize)-blue.1).pow(2) < rad.pow(2)) as i32) as f64;
			img.set_pixel(i, j, (r, g, b));
		}
	}
	img.out(output::Format::PPM);
}

#[test]
fn output_corner() {
	let mut img = output::ImageFile::new(640,480);
	for x in (0..640).rev() {
		for y in 0..480 {
			if x < 50 && y < 50 {
				img.set_pixel(x, y, (1.0, 1.0, 1.0));
			} else {
				img.set_pixel(x, y, (0.0, 0.0, 0.0));
			}
		}
	}
	img.out(output::Format::PPM);
}
