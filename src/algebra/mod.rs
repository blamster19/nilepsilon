use std::ops;
use std::cmp;
use float_eq;

//might want to change to something more/less precise depending on use case
pub type Coord = f64;

#[derive(Default, Clone, Debug)]
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
impl ops::Mul for Vector {
	type Output = Coord;

	fn mul(self, other: Self) ->  Coord{
		self.x * other.x + self.y * other.y + self.z * other.z
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
