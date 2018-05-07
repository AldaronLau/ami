// Aldaron's Memory Interface
// Copyright (c) 2018 Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// mat4.rs

// use std::ops::*;
// use std::fmt::*;

use Vec4;
use Vec3;
use Plane;
use Frustum;

/// A 4x4 Matrix
#[derive(Clone, Copy, PartialEq)]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
	/// A no-op transform (identity matrix).
	pub fn new() -> Mat4 {
		Mat4([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	/// Multiply `self` by a matrix.
	pub fn matrix(self, matrix: [f32; 16]) -> Mat4 {
		self * Mat4(matrix)
	}

	/// Multiply `self` by a scale transformation matrix.
	pub fn scale(self, x: f32, y: f32, z: f32) -> Mat4 {
		self.matrix([
			x,   0.0, 0.0, 0.0,
			0.0, y,   0.0, 0.0,
			0.0, 0.0, z,   0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	/// Multiply `self` by a translation matrix.
	pub fn translate(self, x: f32, y: f32, z: f32) -> Mat4 {
		self.matrix([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			x,   y,   z,   1.0,
		])
	}

	/// Multiply `self` by a rotation matrix.  `x`, `y` and `z` are in PI
	/// Radians.
	pub fn rotate(self, x: f32, y: f32, z: f32) -> Mat4 {
		let num9 = z * ::std::f32::consts::PI;
		let num6 = num9.sin();
		let num5 = num9.cos();
		let num8 = x * ::std::f32::consts::PI;
		let num4 = num8.sin();
		let num3 = num8.cos();
		let num7 = y * ::std::f32::consts::PI;
		let num2 = num7.sin();
		let num = num7.cos();

		let qx = ((num * num4) * num5) + ((num2 * num3) * num6);
		let qy = ((num2 * num3) * num5) - ((num * num4) * num6);
		let qz = ((num * num3) * num6) - ((num2 * num4) * num5);
		let qw = ((num * num3) * num5) + ((num2 * num4) * num6);

		let nx = -qx;
		let ny = -qy;
		let nz = -qz;

		self.matrix([
			qw,nz,qy,nx,
			qz,qw,nx,ny,
			ny,qx,qw,nz,
			qx,qy,qz,qw
		]).matrix([
			qw,nz,qy,qx,
			qz,qw,nx,qy,
			ny,qx,qw,qz,
			nx,ny,nz,qw
		])
	}
}

impl ::std::ops::Mul<Frustum> for Mat4 {
	type Output = Frustum;

	fn mul(self, rhs: Frustum) -> Self::Output {
		Frustum {
			center: self * rhs.center,
			radius: rhs.radius,
			wfov: rhs.wfov,
			hfov: rhs.hfov,
			xrot: rhs.xrot, // TODO
			yrot: rhs.yrot, // TODO
//			near: self * rhs.near,
//			far: self * rhs.far,
//			top: self * rhs.top,
//			bottom: self * rhs.bottom,
//			right: self * rhs.right,
//			left: self * rhs.left,
		}
	}
}

impl ::std::ops::Mul<Plane> for Mat4 {
	type Output = Plane;

	fn mul(self, rhs: Plane) -> Self::Output {
		let facing = rhs.facing.transform_dir(self);
		// translation vector
		let t = Vec3::new(self.0[12], self.0[13], self.0[14]);
		//
		if t == Vec3::zero() {
			return Plane { facing, offset: rhs.offset };
		}
		// Angle between normal and translation
		let mut a = facing.angle(t).abs();

		// If more than full circle, reduce
		while a > ::std::f32::consts::PI * 2.0 {
			a -= ::std::f32::consts::PI * 2.0;
		}

		let mut b = 1.0;

		// If value is over 90° it can be reduced
		if a > ::std::f32::consts::PI / 2.0 {
			// 90°-180° quadrant
			if a < ::std::f32::consts::PI {
				a = ::std::f32::consts::PI - a;
				b = -b;
			// 180°-270° quadrant
			} else if a < 3.0 * ::std::f32::consts::PI / 2.0 {
				a -= ::std::f32::consts::PI;
				b = -b;
			// 270°-360° quadrant
			} else {
				a = (2.0 * ::std::f32::consts::PI) - a;
			}
		}

		// if a == 90°
		let offset = rhs.offset + if a >= ::std::f32::consts::PI / 2.0 {
			0.0
		} else {
			a.cos() * t.mag() * b
		};

		Plane { facing, offset }
	}
}

impl ::std::ops::Mul<Vec3<f32>> for Mat4 {
	type Output = Vec3<f32>;

	/// Transform as a position.
	fn mul(self, rhs: Vec3<f32>) -> Self::Output {
		let x = self.0[0]*rhs.x + self.0[4]*rhs.y + self.0[8]*rhs.z + self.0[12]*1.0;
		let y = self.0[1]*rhs.x + self.0[5]*rhs.y + self.0[9]*rhs.z + self.0[13]*1.0;
		let z = self.0[2]*rhs.x + self.0[6]*rhs.y + self.0[10]*rhs.z + self.0[14]*1.0;

		Vec3::new(x, y, z)
	}
}

impl ::std::ops::Mul<Vec4<f32>> for Mat4 {
	type Output = Vec4<f32>;

	/// Transform as a position.
	fn mul(self, rhs: Vec4<f32>) -> Self::Output {
		let x = self.0[0]*rhs.x + self.0[4]*rhs.y + self.0[8]*rhs.z + self.0[12]*rhs.w;
		let y = self.0[1]*rhs.x + self.0[5]*rhs.y + self.0[9]*rhs.z + self.0[13]*rhs.w;
		let z = self.0[2]*rhs.x + self.0[6]*rhs.y + self.0[10]*rhs.z + self.0[14]*rhs.w;
		let w = self.0[3]*rhs.x + self.0[7]*rhs.y + self.0[11]*rhs.z + self.0[15]*rhs.w;

		Vec4::new(x, y, z, w)
	}
}

impl ::std::ops::Mul<Mat4> for Mat4 {
	type Output = Mat4;

	fn mul(self, rhs: Mat4) -> Self::Output {
		Mat4([
			(self.0[0] * rhs.0[0]) + (self.0[1] * rhs.0[4]) +
			(self.0[2] * rhs.0[8]) + (self.0[3] * rhs.0[12]),
			(self.0[0] * rhs.0[1]) + (self.0[1] * rhs.0[5]) +
			(self.0[2] * rhs.0[9]) + (self.0[3] * rhs.0[13]),
			(self.0[0] * rhs.0[2]) + (self.0[1] * rhs.0[6]) +
			(self.0[2] * rhs.0[10]) + (self.0[3] * rhs.0[14]),
			(self.0[0] * rhs.0[3]) + (self.0[1] * rhs.0[7]) +
			(self.0[2] * rhs.0[11]) + (self.0[3] * rhs.0[15]),

			(self.0[4] * rhs.0[0]) + (self.0[5] * rhs.0[4]) +
			(self.0[6] * rhs.0[8]) + (self.0[7] * rhs.0[12]),
			(self.0[4] * rhs.0[1]) + (self.0[5] * rhs.0[5]) +
			(self.0[6] * rhs.0[9]) + (self.0[7] * rhs.0[13]),
			(self.0[4] * rhs.0[2]) + (self.0[5] * rhs.0[6]) +
			(self.0[6] * rhs.0[10]) + (self.0[7] * rhs.0[14]),
			(self.0[4] * rhs.0[3]) + (self.0[5] * rhs.0[7]) +
			(self.0[6] * rhs.0[11]) + (self.0[7] * rhs.0[15]),

			(self.0[8] * rhs.0[0]) + (self.0[9] * rhs.0[4]) +
			(self.0[10] * rhs.0[8]) + (self.0[11] * rhs.0[12]),
			(self.0[8] * rhs.0[1]) + (self.0[9] * rhs.0[5]) +
			(self.0[10] * rhs.0[9]) + (self.0[11] * rhs.0[13]),
			(self.0[8] * rhs.0[2]) + (self.0[9] * rhs.0[6]) +
			(self.0[10] * rhs.0[10]) + (self.0[11] * rhs.0[14]),
			(self.0[8] * rhs.0[3]) + (self.0[9] * rhs.0[7]) +
			(self.0[10] * rhs.0[11]) + (self.0[11] * rhs.0[15]),

			(self.0[12] * rhs.0[0]) + (self.0[13] * rhs.0[4]) +
			(self.0[14] * rhs.0[8]) + (self.0[15] * rhs.0[12]),
			(self.0[12] * rhs.0[1]) + (self.0[13] * rhs.0[5]) +
			(self.0[14] * rhs.0[9]) + (self.0[15] * rhs.0[13]),
			(self.0[12] * rhs.0[2]) + (self.0[13] * rhs.0[6]) +
			(self.0[14] * rhs.0[10]) + (self.0[15] * rhs.0[14]),
			(self.0[12] * rhs.0[3]) + (self.0[13] * rhs.0[7]) +
			(self.0[14] * rhs.0[11]) + (self.0[15] * rhs.0[15])
		])
	}
}

impl ::std::fmt::Display for Mat4 {
	fn fmt(&self, fmtr: &mut ::std::fmt::Formatter) ->
		::std::result::Result<(), ::std::fmt::Error>
	{
		write!(fmtr, "{:?}", self.0)
	}
}
