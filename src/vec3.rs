// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017  Douglas P. Lau
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::fmt;
use std::ops;

/// 3-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
	/// X coordinate
	pub x: f32,
	/// Y coordinate
	pub y: f32,
	/// Z coordinate
	pub z: f32,
}

impl fmt::Debug for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
	}
}

impl ops::Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Self) -> Self::Output {
		Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl ops::Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Self) -> Self::Output {
		Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl ops::Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, s: f32) -> Self::Output {
		Vec3::new(self.x * s, self.y * s, self.z * s)
	}
}

/*impl<T, U> ops::Mul for Vec3<T> where f64: convert::From<T> {
	type Output = T;

	/// Calculate the cross product of two Vec2
	fn mul(self, other: Self) -> T {
		self.x * other.y - self.y * other.x
	}
}*/

impl ops::Div<f32> for Vec3 {
	type Output = Vec3;

	fn div(self, s: f32) -> Vec3 {
		Vec3::new(self.x / s, self.y / s, self.z / s)
	}
}

impl ops::Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Vec3 {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl Vec3 {
	/// Create a new Vec3
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 { x, y, z }
	}

	/// Find the minimum ordinal value
	pub(crate) fn min_p(self) -> f32 {
		self.x.min(self.y).min(self.z)
	}

	/// Find the maximum ordinal value
	pub(crate) fn max_p(self) -> f32 {
		self.x.max(self.y).max(self.z)
	}

	/// Get the magnitude of a Vec3
	pub fn mag(self) -> f32 {
		(self.x).hypot(self.y).hypot(self.z)
	}

	/// Multiply matrix onto Vec3 (as directional vector)
	pub fn transform_dir(self, rhs: ::Mat4) -> Self {
		let rhs = rhs.to_f32_array();

		let x = rhs[0]*self.x + rhs[4]*self.y + rhs[8]*self.z;
		let y = rhs[1]*self.x + rhs[5]*self.y + rhs[9]*self.z;
		let z = rhs[2]*self.x + rhs[6]*self.y + rhs[10]*self.z;

		Self::new(x, y, z)
	}

	/// Create a zero Vec3
	pub fn zero() -> Self {
		Vec3::new(0.0, 0.0, 0.0)
	}

	/// Find the midpoint between two Vec3
	pub fn midpoint(self, other: Self) -> Self {
		let x = (self.x + other.x) / 2.0;
		let y = (self.y + other.y) / 2.0;
		let z = (self.z + other.z) / 2.0;
		Vec3::new(x, y, z)
	}

	/// Calculate the distance squared between two Vec3
	pub fn dist_sq(self, other: Self) -> f32 {
		let dx = other.x - self.x;
		let dy = other.y - self.y;
		let dz = other.z - self.z;
		dx * dx + dy * dy + dz * dz
	}

	/// The recipricol (inverse) of the vector.
	pub fn recip(self) -> Self {
		Vec3::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
	}

	/// Calculate the dot product of two `Vec3`s
	pub fn dot(&self, other: Vec3) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	/// Normalize a Vec3
	pub fn normalize(self) -> Self {
		let m = self.mag();
		if m > 0.0 {
			self / m
		} else {
			Vec3::zero()
		}
	}

	/// Calculate angle between 2 Vec3's
	pub fn angle(&self, other: Vec3) -> f32 {
		let mag1 = (self.x as f64)
			.hypot(self.y as f64)
			.hypot(self.z as f64);
		let mag2 = (other.x as f64)
			.hypot(other.y as f64)
			.hypot(other.z as f64);
		let dot = ((self.x as f64) * (other.x as f64))
			+ ((self.y as f64) * (other.y as f64))
			+ ((self.z as f64) * (other.z as f64));

		(dot / (mag1 * mag2)).acos() as f32
	}
}
