// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

/// 3D vector macro constructor.
///
/// `(0.0, 0.0, 0.0)`: `vector!()`
///
/// `(1.0, 1.0, 1.0)`: `vector!(1.0)`
///
/// `(0.0, 1.0, 2.0)`: `vector!(0.0, 1.0, 2.0)`
#[macro_export] macro_rules! vector {
	() => {
		$crate::Vector::default()
	};

	($expr:expr) => {
		$crate::Vector::from($expr)
	};

	($x:expr, $y:expr, $z:expr) => {
		$crate::Vector::new($x as f32, $y as f32, $z as f32)
	};
}

/// 4x4 matrix macro constructor.
///
/// Identity: `matrix!()`
/// 
/// Array:
/// ```
/// matrix!([
/// 	[1.0, 0.0, 0.0, 0.0],
/// 	[0.0, 1.0, 0.0, 0.0],
/// 	[0.0, 0.0, 1.0, 0.0],
/// 	[0.0, 0.0, 0.0, 1.0],
/// ])
///
/// matrix!(
/// 	1.0, 0.0, 0.0, 0.0,
/// 	0.0, 1.0, 0.0, 0.0,
/// 	0.0, 0.0, 1.0, 0.0,
/// 	0.0, 0.0, 0.0, 1.0,
/// )
#[macro_export] macro_rules! matrix {
	() => {
		$crate::Matrix {
			m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
			m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
			m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
			m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
		}
	};

	($expr:expr) => {
		$crate::Matrix::from($expr)
	};

	(
		$m00:expr, $m01:expr, $m02:expr, $m03:expr,
		$m10:expr, $m11:expr, $m12:expr, $m13:expr,
		$m20:expr, $m21:expr, $m22:expr, $m23:expr,
		$m30:expr, $m31:expr, $m32:expr, $m33:expr,
	) => {
		$crate::Matrix::new(
			$m00 as f32, $m01 as f32, $m02 as f32, $m03 as f32,
			$m10 as f32, $m11 as f32, $m12 as f32, $m13 as f32,
			$m20 as f32, $m21 as f32, $m22 as f32, $m23 as f32,
			$m30 as f32, $m31 as f32, $m32 as f32, $m33 as f32,
		)
	};
}
