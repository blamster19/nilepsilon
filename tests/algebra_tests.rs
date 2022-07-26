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
	let v3 = (14.0 as algebra::Coord).sqrt();
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
