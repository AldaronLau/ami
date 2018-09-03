// Copyright Jeron A. Lau 2017-2018.
// Copyright Douglas Lau 2017
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use cgmath;
use std::{fmt, mem, ops};

use cgmath::{InnerSpace, Rotation3};
use *;

/// Single-precision quaternion.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
#[repr(C)]
pub struct Rotation {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub s: f32,
}

impl fmt::Display for Rotation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {}, {}; {})", self.x, self.y, self.z, self.s)
	}
}

impl Rotation {
	/// Identity constructor.
	pub fn identity() -> Self {
		Rotation { x: 0.0, y: 0.0, z: 0.0, s: 1.0 }
	}

	/// Constructor for a rotation defined by a set of Euler angles
	///
	/// The rotation order is Z, then X, then Y. From the point of the
	/// object, this is equivalent to a yaw in `angles.y`, a pitch in
	/// `angles.x`, and a roll in `angles.z`.
	pub fn euler(angles: Vector) -> Self {
		let roll = Rotation::new(vector!(0.0, 0.0, 1.0), angles.z);
		let pitch = Rotation::new(vector!(1.0, 0.0, 0.0), angles.x);
		let yaw = Rotation::new(vector!(0.0, 1.0, 0.0), angles.y);
		roll * pitch * yaw
	}

	/// Create a new `Rotation` from axis and angle.
	pub fn new(axis: Vector, angle: f32) -> Self {
		let q = cgmath::Quaternion::from_axis_angle(
			cgmath::Vector3::new(axis.x, axis.y, axis.z).normalize(),
			cgmath::Rad(angle),
		);

		Rotation { x: q.v.x, y: q.v.y, z: q.v.z, s: q.s }
	}

	/// Return the application of the rotation represented by this quaternion
	/// to the vector argument.
	pub fn rotate(&self, vector: Vector) -> Vector {
		use cgmath::Rotation;
		let rotation = cgmath::Quaternion::new(self.s, self.x, self.y, self.z);
		let point = cgmath::Point3::new(vector.x, vector.y, vector.z);
		let result = rotation.rotate_point(point);
		vector!(result.x, result.y, result.z)
	}

	/// Add another `Rotation` after the current `Rotation`.
	pub fn then(self, rhs: Self) -> Self {
		// TODO: check to make sure this order is correct.
		// (that it is not `self * rhs`).  Internet search is of no help
		// so experimentation is the only way to find out.
		rhs * self
	}
}

impl ops::Mul<f32> for Rotation {
	type Output = Rotation;
	fn mul(mut self, rhs: f32) -> Rotation {
		self.s *= rhs;
		self
	}
}

impl ops::Mul<Rotation> for Rotation {
	type Output = Rotation;
	fn mul(self, rhs: Rotation) -> Rotation {
		let a: &cgmath::Quaternion<f32> = self.as_ref().into();
		let b: &cgmath::Quaternion<f32> = rhs.as_ref().into();
		let q = a * b;

		Rotation { x: q.v.x, y: q.v.y, z: q.v.z, s: q.s }
	}
}

impl ops::MulAssign<Rotation> for Rotation {
	fn mul_assign(&mut self, rhs: Rotation) {
		*self = *self * rhs;
	}
}

impl Default for Rotation {
	fn default() -> Self {
		Self::identity()
	}
}

impl AsRef<[f32; 4]> for Rotation {
	fn as_ref(&self) -> &[f32; 4] {
		unsafe {
			mem::transmute(self)
		}
	}
}

impl From<[f32; 4]> for Rotation {
	fn from(q: [f32; 4]) -> Self {
		Rotation { x: q[0], y: q[1], z: q[2], s: q[3] }
	}
}

impl Into<[f32; 4]> for Rotation {
	fn into(self) -> [f32; 4] {
		[self.x, self.y, self.z, self.s]
	}
}
