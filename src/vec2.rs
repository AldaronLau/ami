// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017  Douglas P. Lau
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::fmt;
use std::ops;

/// Calculate linear interpolation of two values
///
/// The t value should be between 0 and 1.
fn float_lerp(a: f32, b: f32, t: f32) -> f32 {
	b + (a - b) * t
}

/// Calculate intersection point of two lines.
///
/// Returns None if the lines are colinear.
#[allow(unused)]
fn intersection(a0: Vec2, a1: Vec2, b0: Vec2, b1: Vec2) -> Option<Vec2> {
	let av = a0 - a1;
	let bv = b0 - b1;
	let den = av * bv;
	if den != 0f32 {
		let ca = a0 * a1;
		let cb = b0 * b1;
		let xn = bv.x * ca - av.x * cb;
		let yn = bv.y * ca - av.y * cb;
		Some(Vec2::new(xn / den, yn / den))
	} else {
		None
	}
}

/// 2-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
	/// X coordinate
	pub x: f32,
	/// Y coordinate
	pub y: f32,
}

impl fmt::Debug for Vec2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({},{})", self.x, self.y)
	}
}

impl ops::Add for Vec2 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Vec2::new(self.x + other.x, self.y + other.y)
	}
}

impl ops::Sub for Vec2 {
	type Output = Self;

	fn sub(self, other: Self) -> Self{
		Vec2::new(self.x - other.x, self.y - other.y)
	}
}

impl ops::Mul<f32> for Vec2 {
	type Output = Self;

	fn mul(self, s: f32) -> Self {
		Vec2::new(self.x * s, self.y * s)
	}
}

impl ops::Mul for Vec2 {
	type Output = f32;

	/// Calculate the cross product of two Vec2
	fn mul(self, other: Self) -> f32 {
		self.x * other.y - self.y * other.x
	}
}

impl ops::Div<f32> for Vec2 {
	type Output = Self;

	fn div(self, s: f32) -> Self {
		Vec2::new(self.x / s, self.y / s)
	}
}

impl ops::Neg for Vec2 {
	type Output = Self;

	fn neg(self) -> Self {
		Vec2::new(-self.x, -self.y)
	}
}

#[allow(unused)]
impl Vec2 {
	/// Create a new Vec2
	pub fn new(x: f32, y: f32) -> Self {
		Vec2 { x, y }
	}

	/// Create a zero Vec2
	pub fn zero() -> Self {
		Vec2::new(0f32, 0f32)
	}

	/// Calculate the dot product of two `Vec2`s
	pub fn dot(&self, other: Vec2) -> f32 {
		self.x * other.x + self.y * other.y
	}

	/// Get the magnitude of a Vec2
	pub fn mag(self) -> f32 {
		self.x.hypot(self.y)
	}

	/// Normalize a Vec2
	pub fn normalize(self) -> Self {
		let m = self.mag();
		if m > 0.0 {
			self / m
		} else {
			Vec2::zero()
		}
	}

	/// Calculate the distance squared between two Vec2
	pub fn dist_sq(self, other: Self) -> f32 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		dx * dx + dy * dy
	}

	/// Calculate the distance between two Vec2
	pub fn dist(self, other: Self) -> f32 {
		self.dist_sq(other).sqrt()
	}

	/// Find the midpoint between two Vec2
	pub fn midpoint(self, other: Self) -> Self {
		let x = (self.x + other.x) / 2f32;
		let y = (self.y + other.y) / 2f32;
		Vec2::new(x, y)
	}

	/// Create a left-hand perpendicular Vec2
	pub fn left(self) -> Self {
		Vec2::new(-self.y, self.x)
	}

	/// Create a right-hand perpendicular Vec2
	pub fn right(self) -> Self {
		Vec2::new(self.y, -self.x)
	}

	/// Calculate winding order for two Vec2.
	///
	/// The Vec2 should be initialized as edges pointing toward the same vertex.
	/// Returns true if the winding order is widdershins (counter-clockwise).
	pub fn widdershins(self, other: Self) -> bool {
		// Cross product (with Z zero) is used to determine the winding order.
		(self.x * other.y) > (other.x * self.y)
	}

	/// Calculate linear interpolation of two Vec2
	pub fn lerp(self, other: Self, t: f32) -> Self {
		let x = float_lerp(self.x, other.x, t);
		let y = float_lerp(self.y, other.y, t);
		Vec2::new(x, y)
	}

	/// Calculate angle between 2 Vec2's (on a plane they both lie on)
	pub fn angle(&self, other: Vec2) -> f32 {
		(self.dot(other) / (self.mag() * other.mag())).acos()
	}
}

#[test]
fn test_vec2() {
	let a = Vec2::new(2f32, 1f32);
	let b = Vec2::new(3f32, 4f32);
	assert!(a + b == Vec2::new(5f32, 5f32));
	assert!(b - a == Vec2::new(1f32, 3f32));
	assert!(a * 2f32 == Vec2::new(4f32, 2f32));
	assert!(a / 2f32 == Vec2::new(1f32, 0.5f32));
	assert!(-a == Vec2::new(-2f32, -1f32));
	assert!(b.mag() == 5f32);
	assert!(a.normalize() == Vec2::new(0.8944272f32, 0.4472136f32));
	assert!(a.dist_sq(b) == 10f32);
	assert!(b.dist(Vec2::new(0f32, 0f32)) == 5f32);
	assert!(a.midpoint(b) == Vec2::new(2.5f32, 2.5f32));
	assert!(a.left() == Vec2::new(-1f32, 2f32));
	assert!(a.right() == Vec2::new(1f32, -2f32));
}
