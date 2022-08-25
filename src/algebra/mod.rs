use float_eq;
use std::cmp;
use std::ops;

//might want to change to something more/less precise depending on use case
pub type Scalar = f64;

#[derive(Default, Clone, Debug, Copy)]
pub struct Vector {
	pub x: Scalar,
	pub y: Scalar,
	pub z: Scalar,
}

impl Vector {
	pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Vector {
		Vector { x, y, z }
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
	type Output = Scalar;

	fn mul(self, other: Self) -> Scalar {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
}

//multiply by scalar
//Vector * Scalar
impl ops::Mul<Scalar> for Vector {
	type Output = Self;

	fn mul(self, k: Scalar) -> Self::Output {
		Self {
			x: self.x * k,
			y: self.y * k,
			z: self.z * k,
		}
	}
}

//Scalar * Vector
impl ops::Mul<Vector> for Scalar {
	type Output = Vector;

	fn mul(self, v: Vector) -> Self::Output {
		Vector {
			x: v.x * self,
			y: v.y * self,
			z: v.z * self,
		}
	}
}

//divide by scalar
impl ops::Div<Scalar> for Vector {
	type Output = Self;

	fn div(self, k: Scalar) -> Self::Output {
		Self {
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
		float_eq::float_eq!(self.x, other.x, rmax <= Scalar::EPSILON)
			&& float_eq::float_eq!(self.y, other.y, rmax <= Scalar::EPSILON)
			&& float_eq::float_eq!(self.z, other.z, rmax <= Scalar::EPSILON)
	}
}

impl Vector {
	pub fn norm_sqr(self) -> Scalar {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn norm(self) -> Scalar {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalize(self) -> Vector {
		self / self.norm()
	}
}

#[derive(Clone, Copy)]
pub struct WavelengthBunch(pub Scalar, pub Scalar, pub Scalar, pub Scalar);

impl ops::Add for WavelengthBunch {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		WavelengthBunch(
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2,
			self.3 + other.3,
		)
	}
}

impl ops::Mul<WavelengthBunch> for WavelengthBunch {
	type Output = Self;

	fn mul(self, other: Self) -> WavelengthBunch {
		WavelengthBunch(
			self.0 * other.0,
			self.1 * other.1,
			self.2 * other.2,
			self.3 * other.3,
		)
	}
}

pub struct Basis {
	// vectors given in world basis
	pub u: Vector,
	pub v: Vector,
	pub w: Vector,
}

impl Basis {
	pub fn new(u: Vector, v: Vector, w: Vector) -> Self {
		Self { u, v, w }
	}
	pub fn basis_to_world(&self, vector: Vector) -> Vector {
		self.u * vector.x + self.v * vector.y + self.w * vector.z
	}
}
