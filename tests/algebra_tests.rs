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
