use std::ops;
use std::cmp;
use float_eq;

//might want to change to something more/less precise depending on use case
pub type Coord = f64;

#[derive(Default, Clone, Debug, Copy)]
pub struct Vector {
	pub x: Coord,
	pub y: Coord,
	pub z: Coord,
}

impl Vector {
	fn new(x: Coord, y: Coord, z: Coord) -> Vector {
		Vector {
			x,
			y,
			z,
		}
	}
}

impl ops::Add for Vector {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Vector {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::Sub for Vector {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Vector {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

//scalar product
impl ops::Mul<Vector> for Vector {
	type Output = Coord;

	fn mul(self, other: Self) ->  Coord{
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

//multiply by scalar
impl ops::Mul<Coord> for Vector {
	type Output = Vector;

	fn mul(self, k: Coord) -> Vector {
		Vector {
			x: self.x * k,
			y: self.y * k,
			z: self.z * k,
		}
	}
}

//divide by scalar
impl ops::Div<Coord> for Vector {
	type Output = Vector;

	fn div(self, k: Coord) -> Vector {
		Vector {
			x: self.x / k,
			y: self.y / k,
			z: self.z / k,
		}
	}
}

//vector product
impl ops::Rem for Vector {
	type Output = Self;

	fn rem(self, other: Self) -> Self {
		Vector {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}
}

//equality
impl cmp::PartialEq for Vector {
	fn eq(&self, other: &Self) -> bool {
		float_eq::float_eq!(self.x, other.x, rmax <= Coord::EPSILON) &&
		float_eq::float_eq!(self.y, other.y, rmax <= Coord::EPSILON) &&
		float_eq::float_eq!(self.z, other.z, rmax <= Coord::EPSILON)
	}
}

impl Vector {
	fn norm_sqr(self) -> Coord {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	fn norm(self) -> Coord {	
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	fn normalize(self) -> Vector {
		self/self.norm()
	}
}
