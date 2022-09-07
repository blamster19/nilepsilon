use nilepsilon::algebra;

static v1: algebra::Vector = algebra::Vector {
	x: 1.0,
	y: 2.0,
	z: 3.0,
};
static v2: algebra::Vector = algebra::Vector {
	x: 4.0,
	y: 5.0,
	z: 6.0,
};

#[test]
fn vector_add() {
	let v3 = algebra::Vector {
		x: 5.0,
		y: 7.0,
		z: 9.0,
	};
	let result = v1 + v2;
	assert_eq!(result, v3);
}

#[test]
fn vector_sub() {
	let v3 = algebra::Vector {
		x: -3.0,
		y: -3.0,
		z: -3.0,
	};
	let result = v1 - v2;
	assert_eq!(result, v3);
}

#[test]
fn vector_scalar_prod() {
	let v3 = 32.0;
	let result = v1 * v2;
	assert_eq!(result, v3);
}

#[test]
fn vector_vector_prod() {
	let v3 = algebra::Vector {
		x: -3.0,
		y: 6.0,
		z: -3.0,
	};
	let result = v1 % v2;
	assert_eq!(result, v3);
}

#[test]
fn vector_mul_by_scalar() {
	let v3 = algebra::Vector {
		x: 2.0,
		y: 4.0,
		z: 6.0,
	};
	let result = 2.0 * v1;
	assert_eq!(result, v3);
}

#[test]
fn vector_div_by_scalar() {
	let v3 = algebra::Vector {
		x: 0.5,
		y: 1.0,
		z: 1.5,
	};
	let result = v1 / 2.0;
	assert_eq!(result, v3);
}

#[test]
fn vector_norm() {
	let v3 = (14.0 as algebra::Scalar).sqrt();
	let result = v1.norm();
	assert_eq!(result, v3);
}

#[test]
fn vector_norm_sqr() {
	let v3 = 14.0;
	let result = v1.norm_sqr();
	assert_eq!(result, v3);
}

#[test]
fn vector_normalize() {
	let v3 = algebra::Vector {
		x: 0.2672612419124244,
		y: 0.5345224838248488,
		z: 0.8017837257372732,
	};
	let result = v1.normalize();
	assert_eq!(result, v3);
}

#[test]
fn basis() {
	let b = algebra::Basis::new(
		algebra::Vector::new(2.0, 1.0, 2.0) / 3.0,
		algebra::Vector::new(-2.0, 2.0, 1.0) / 3.0,
		algebra::Vector::new(1.0, 2.0, -2.0) / 3.0,
	);
	let v3 = algebra::Vector::new(10.0 / 3.0, 5.0 / 3.0, -1.0 / 3.0);
	let result = b.world_to_basis(v1);
	assert_eq!(true, (result - v3).norm() < 0.000000001);
}

#[test]
fn spherical() {
	let b = algebra::Basis::new(
		algebra::Vector::new(1.0, 0.0, 0.0),
		algebra::Vector::new(0.0, 1.0, 0.0),
		algebra::Vector::new(0.0, 0.0, 1.0),
	);
	let result: (algebra::Scalar, algebra::Scalar) = b.basis_to_spherical(v1.normalize());
	assert_eq!(result, (0.6405223126794245, 1.1071487177940904));
}
