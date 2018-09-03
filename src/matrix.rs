// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use cgmath;
use std::{fmt, mem, ops};
use *;

/// 4x4 Matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
#[repr(C)]
pub struct Matrix {
	pub m00: f32, pub m01: f32, pub m02: f32, pub m03: f32,
	pub m10: f32, pub m11: f32, pub m12: f32, pub m13: f32,
	pub m20: f32, pub m21: f32, pub m22: f32, pub m23: f32,
	pub m30: f32, pub m31: f32, pub m32: f32, pub m33: f32,
}

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"({}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {}; {}, {}, {}, {})",
			self.m00, self.m01, self.m02, self.m03,
			self.m10, self.m11, self.m12, self.m13,
			self.m20, self.m21, self.m22, self.m23,
			self.m30, self.m31, self.m32, self.m33,
		)
	}
}

impl Matrix {
	/// Scale, then rotate Quaternion (axis, angle), then translate.
	#[inline(always)]
	pub fn srt(self, scale: Vector, rotate: Rotation, translate: Vector)
		-> Self
	{
		self.s(scale).r(rotate).t(translate)
	}

	/// Rotate Quaternion (axis, angle), then translate.
	#[inline(always)]
	pub fn rt(self, rotate: Rotation, translate: Vector) -> Self {
		self.r(rotate).t(translate)
	}

	/// Scale, then translate.
	#[inline(always)]
	pub fn st(self, scale: Vector, translate: Vector) -> Self {
		self.s(scale).t(translate)
	}

	/// Translate.
	#[inline(always)]
	pub fn t(self, translate: Vector) -> Self {
		let t: [[f32; 4]; 4] = cgmath::Matrix4::from_translation(
			cgmath::Vector3::new(
				translate.x, translate.y, translate.z
			)
		).into();

		self.m(Matrix::from(t))
	}

	/// Scale.
	#[inline(always)]
	pub fn s(self, scale: Vector) -> Self {
		let t: [[f32; 4]; 4] = cgmath::Matrix4::from_nonuniform_scale(
			scale.x, scale.y, scale.z
		).into();

		self.m(Matrix::from(t))
	}

	/// Rotate Quaternion (axis, angle).
	#[inline(always)]
	pub fn r(self, rotation: Rotation) -> Self {
		let t: [[f32; 4]; 4] = cgmath::Matrix4::from(
			cgmath::Quaternion::new(
				rotation.s, rotation.x, rotation.y, rotation.z
			)
		).into();

		self.m(Matrix::from(t))
	}

	/// Multiply by a custom matrix
	#[inline(always)]
	pub fn m(self, matrix: Self) -> Self {
		matrix * self
	}

	/// Full constructor.
	pub fn new(
		m00: f32, m01: f32, m02: f32, m03: f32,
		m10: f32, m11: f32, m12: f32, m13: f32,
		m20: f32, m21: f32, m22: f32, m23: f32,
		m30: f32, m31: f32, m32: f32, m33: f32,
	) -> Self {
		Matrix {
			m00, m01, m02, m03,
			m10, m11, m12, m13,
			m20, m21, m22, m23,
			m30, m31, m32, m33,
		}
	}

	/// Diagonal constructor.
	pub fn diagonal(di: f32) -> Self {
		Matrix::tridiagonal(0.0, di, 0.0)
	}

	/// Tri-diagonal constructor.
	pub fn tridiagonal(lo: f32, di: f32, up: f32) -> Self {
		Matrix::new(
			di, up, 0., 0.,
			lo, di, up, 0.,
			0., lo, di, up,
			0., 0., lo, di,
		)
	}

	/// Orthographic projection matrix constructor.
	pub fn orthographic_projection(
		left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32
	) -> Self {
		let ortho = cgmath::ortho(left, right, bottom, top, near, far);

		Matrix::new(
			ortho.x.x, ortho.x.y, ortho.x.z, ortho.x.w,
			ortho.y.x, ortho.y.y, ortho.y.z, ortho.y.w,
			ortho.z.x, ortho.z.y, ortho.z.z, ortho.z.w,
			ortho.w.x, ortho.w.y, ortho.w.z, ortho.w.w,
		)
	}

	/// Finite perspective projection matrix constructor.
	pub fn finite_perspective_projection(
		fovy: f32, aspect: f32, near: f32, far: f32
	) -> Self {
		let pp = cgmath::perspective(cgmath::Rad(fovy), aspect, near, far);

		Matrix::new(
			pp.x.x, pp.x.y, pp.x.z, pp.x.w,
			pp.y.x, pp.y.y, pp.y.z, pp.y.w,
			pp.z.x, pp.z.y, pp.z.z, pp.z.w,
			pp.w.x, pp.w.y, pp.w.z, pp.w.w,
		)
	}

	/// Computes the matrix determinant.
	pub fn determinant(self) -> f32 {
		use cgmath::SquareMatrix;
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		a.determinant()
	}

	/// Computes the matrix trace.
	pub fn trace(self) -> f32 {
		use cgmath::SquareMatrix;
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		a.trace()
	}

	/// Computes the matrix inverse.
	///
	/// ## Panics
	///
	/// Panics if the matrix has no inverse (i.e. has zero determinant).
	pub fn inverse(self) -> Matrix {
		self.try_invert().unwrap()
	}

	/// Returns the matrix transpose.
	pub fn transpose(self) -> Matrix {
		use cgmath::SquareMatrix;
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let mut b = *a;
		b.transpose_self();
		let m: [[f32; 4]; 4] = b.into();
		m.into()
	}

	/// Attempts to compute the matrix inverse, returning `None` if the matrix is
	/// non-invertible (i.e. has zero determinant).
	pub fn try_invert(self) -> Option<Matrix> {
		use cgmath::SquareMatrix;
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		a.invert().map(|inv| {
			let b: [[f32; 4]; 4] = inv.into();
			b.into()
		})
	}
}

impl ops::Add<Matrix> for Matrix {
	type Output = Matrix;
	fn add(self, rhs: Matrix) -> Self::Output {
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let b: &cgmath::Matrix4<f32> = rhs.as_ref().into();
		let m: [[f32; 4]; 4] = (a + b).into();
		m.into()
	}
}

impl ops::Sub<Matrix> for Matrix {
	type Output = Matrix;
	fn sub(self, rhs: Matrix) -> Self::Output {
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let b: &cgmath::Matrix4<f32> = rhs.as_ref().into();
		let m: [[f32; 4]; 4] = (a - b).into();
		m.into()
	}
}

impl ops::Mul<f32> for Matrix {
	type Output = Matrix;
	fn mul(self, rhs: f32) -> Self::Output {
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let v: [[f32; 4]; 4] = (a * rhs).into();
		v.into()
	}
}

impl ops::Mul<(Vector, f32)> for Matrix {
	type Output = Vector;
	fn mul(self, rhs: (Vector, f32)) -> Self::Output {
		let p = [rhs.0.x, rhs.0.y, rhs.0.z, rhs.1];
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let b: &cgmath::Vector4<f32> = (&p).into();
		let v: [f32; 4] = (a * b).into();

		Vector::new(v[0], v[1], v[2])
	}
}

impl<'a> ops::Mul<(Vector, f32)> for &'a Matrix {
	type Output = Vector;
	fn mul(self, rhs: (Vector, f32)) -> Self::Output {
		let p = [rhs.0.x, rhs.0.y, rhs.0.z, rhs.1];
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let b: &cgmath::Vector4<f32> = (&p).into();
		let v: [f32; 4] = (a * b).into();

		Vector::new(v[0], v[1], v[2])
	}
}

impl ops::Mul<Matrix> for f32 {
	type Output = Matrix;
	fn mul(self, rhs: Matrix) -> Self::Output {
		ops::Mul::mul(rhs, self)
	}
}

impl ops::Mul<Matrix> for Matrix {
	type Output = Matrix;
	fn mul(self, rhs: Matrix) -> Self::Output {
		let a: &cgmath::Matrix4<f32> = self.as_ref().into();
		let b: &cgmath::Matrix4<f32> = rhs.as_ref().into();
		let v: [[f32; 4]; 4] = (a * b).into();
		v.into()
	}
}

impl Default for Matrix {
	fn default() -> Self {
		matrix!()
	}
}

impl AsRef<[[f32; 4]; 4]> for Matrix {
	fn as_ref(&self) -> &[[f32; 4]; 4] {
		unsafe {
			mem::transmute(self)
		}
	}
}

impl From<[[f32; 4]; 4]> for Matrix {
	fn from(array: [[f32; 4]; 4]) -> Self {
		unsafe {
			mem::transmute(array)
		}
	}
}

impl Into<[[f32; 4]; 4]> for Matrix {
	fn into(self) -> [[f32; 4]; 4] {
		unsafe {
			mem::transmute(self)
		}
	}
}

impl From<f32> for Matrix {
	fn from(arg: f32) -> Self {
		Matrix::diagonal(arg)
	}
}

impl Into<[f32;16]> for Matrix {
	fn into(self) -> [f32; 16] {
		[
			self.m00, self.m01, self.m02, self.m03,
			self.m10, self.m11, self.m12, self.m13,
			self.m20, self.m21, self.m22, self.m23,
			self.m30, self.m31, self.m32, self.m33,
		]
	}
}
