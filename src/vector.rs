// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use cgmath;
use std::{fmt, mem, ops};

/// Single-precision 3D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(missing_docs)]
#[repr(C)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl From<f32> for Vector {
	fn from(arg: f32) -> Self {
		Self::new(arg, arg, arg)
	}
}

impl fmt::Display for Vector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", (self.x, self.y, self.z))
	}
}

impl Vector {
	/// Full constructor.
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Vector { x, y, z }
	}

	/// Zero constructor.
	pub fn zero() -> Self {
		Default::default()
	}

	/// Returns the cross product of two vectors.
	pub fn cross(self, rhs: Self) -> Self {
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let b: &cgmath::Vector3<f32> = rhs.as_ref().into();
		let v: [f32; 3] = a.cross(*b).into();
		v.into()
	}

	/// Returns the acute angle between two vectors.
	///
	/// # Panics
	///
	/// Panics if `self` is the zero vector.
	pub fn angle(self, rhs: Self) -> f32 {
		(self.dot(rhs) / self.length()).acos()
	}

	/// Returns the dot product of two vectors.
	pub fn dot(self, rhs: Vector) -> f32 {
		use cgmath::InnerSpace;
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let b: &cgmath::Vector3<f32> = rhs.as_ref().into();
		a.dot(*b)
	}

	/// Returns the length (magnitude) of the vector.
	pub fn length(self) -> f32 {
		use cgmath::InnerSpace;
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		a.magnitude()
	}

	/// Returns the squared length of the vector.
	pub fn squared_length(self) -> f32 {
		use cgmath::InnerSpace;
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		a.magnitude2()
	}

	/// Scales the vector to unit length.
	///
	/// ## Panics
	///
	/// Panics if the vector is zero.
	pub fn normalize(self) -> Vector {
		use cgmath::InnerSpace;
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let v: [f32; 3] = a.normalize().into();
		v.into()
	}
}

impl ops::Add<Vector> for Vector {
	type Output = Vector;
	fn add(self, rhs: Vector) -> Self::Output {
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let b: &cgmath::Vector3<f32> = rhs.as_ref().into();
		let v: [f32; 3] = (a + b).into();
		v.into()
	}
}

impl ops::AddAssign<Vector> for Vector {
	fn add_assign(&mut self, rhs: Vector) {
		*self = *self + rhs;
	}
}

impl ops::Sub<Vector> for Vector {
	type Output = Vector;
	fn sub(self, rhs: Vector) -> Self::Output {
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let b: &cgmath::Vector3<f32> = rhs.as_ref().into();
		let v: [f32; 3] = (a - b).into();
		v.into()
	}
}

impl ops::SubAssign<Vector> for Vector {
	fn sub_assign(&mut self, rhs: Vector) {
		*self = *self - rhs;
	}
}

impl ops::Mul<Vector> for f32 {
	type Output = Vector;
	fn mul(self, arg: Vector) -> Self::Output {
		let a: &cgmath::Vector3<f32> = arg.as_ref().into();
		let v: [f32; 3] = (self * a).into();
		v.into()
	}
}

impl ops::Mul<f32> for Vector {
	type Output = Vector;
	fn mul(self, arg: f32) -> Self::Output {
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let v: [f32; 3] = (arg * a).into();
		v.into()
	}
}

impl ops::MulAssign<f32> for Vector {
	fn mul_assign(&mut self, rhs: f32) {
		*self = *self * rhs;
	}
}

impl ops::Div<f32> for Vector {
	type Output = Vector;
	fn div(self, arg: f32) -> Self::Output {
		let a: &cgmath::Vector3<f32> = self.as_ref().into();
		let v: [f32; 3] = (a / arg).into();
		v.into()
	}
}

impl ops::DivAssign<f32> for Vector {
	fn div_assign(&mut self, rhs: f32) {
		*self = *self / rhs;
	}
}

impl ops::Neg for Vector {
	type Output = Vector;

	fn neg(self) -> Vector {
		Vector { x: -self.x, y: -self.y, z: -self.z }
	}
}

impl AsRef<[f32; 3]> for Vector {
	fn as_ref(&self) -> &[f32; 3] {
		unsafe {
			mem::transmute(self)
		}
	}
}

impl From<[f32; 3]> for Vector {
	fn from(array: [f32; 3]) -> Self {
		unsafe {
			mem::transmute(array)
		}
	}
}

impl Into<[f32; 3]> for Vector {
	fn into(self) -> [f32; 3] {
		unsafe {
			mem::transmute(self)
		}
	}
}
