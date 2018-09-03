// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::{ fmt, ops };
use *;

/// Single-precision bounding box
#[derive(Clone, Copy)]
pub struct BBox {
	pub(crate) min: Vector,
	pub(crate) max: Vector,
}

impl fmt::Debug for BBox {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} → {:?}", self.min, self.max)
	}
}

impl ops::Sub<Vector> for BBox {
	type Output = BBox;

	fn sub(self, other: Vector) -> Self::Output {
		BBox::new(self.min - other, self.max - other)
	}
}

impl ops::Add<Vector> for BBox {
	type Output = BBox;

	fn add(self, other: Vector) -> Self::Output {
		BBox::new(self.min + other, self.max + other)
	}
}

impl Into<BCube> for BBox {
	fn into(self) -> BCube {
		let span = self.max - self.min;

		BCube {
			center: self.center(),
			half_len: span.x.max(span.y).max(span.z) / 2.0,
		}
	}
}

impl BBox {
	/// Create an new `BBox` at position `p`.
	pub fn new(min: Vector, max: Vector) -> BBox {
		assert!(min.x <= max.x);
		assert!(min.y <= max.y);
		assert!(min.z <= max.z);

		BBox { min, max }
	}

	/// Check if `BBox` collides with `other` `BBox`.
	pub fn collide(&self, other: BBox) -> bool {
		   other.max.x >= self.min.x
		&& self.max.x >= other.min.x
		&& other.max.y >= self.min.y
		&& self.max.y >= other.min.y
		&& other.max.z >= self.min.z
		&& self.max.z >= other.min.z
	}

	/// Check if `BBox` collides with `BCube`.
	pub fn collide_bcube(&self, bcube: BCube) -> bool {
		let (max, min) = bcube.to_point_pair();
		self.collide(BBox::new(min, max))
	}

	/// Get which sides are the farthest away from the bbox (to extend).
	pub(crate) fn bcube_sides(&self, bcube: BCube) -> (bool, bool, bool) {
		let (max, min) = bcube.to_point_pair();
		let cube = BBox::new(min, max);

		let lt_dist = self.max.x - cube.max.x;
		let rt_dist = cube.min.x - self.min.x;
		let up_dist = self.max.y - cube.max.y;
		let dn_dist = cube.min.y - self.min.y;
		let nr_dist = self.max.z - cube.max.z;
		let fr_dist = cube.min.z - self.min.z;

		(rt_dist <= lt_dist, dn_dist <= up_dist, fr_dist <= nr_dist)
	}

	/// Check if `BBox` collides with point `p`.
	pub fn collide_vec3(&self, p: Vector) -> bool {
		(p.x >= self.min.x) &&
		(p.x <= self.max.x) &&
		(p.y >= self.min.y) &&
		(p.y <= self.max.y) &&
		(p.z >= self.min.z) &&
		(p.z <= self.max.z)
	}

	/// Get all 8 points of the `BBox`.
	pub fn all_points(&self) -> [Vector; 8] {
		[
			Vector::new(self.min.x, self.min.y, self.min.z),
			Vector::new(self.min.x, self.min.y, self.max.z),
			Vector::new(self.min.x, self.max.y, self.min.z),
			Vector::new(self.min.x, self.max.y, self.max.z),
			Vector::new(self.max.x, self.min.y, self.min.z),
			Vector::new(self.max.x, self.min.y, self.max.z),
			Vector::new(self.max.x, self.max.y, self.min.z),
			Vector::new(self.max.x, self.max.y, self.max.z),
		]
	}

	/// Get all 6 sides of the `BBox` as points.
	pub fn side_points(&self) -> [Vector; 6] {
		let center = self.center();

		[
			Vector::new(self.min.x, center.y, center.z),
			Vector::new(center.x, self.min.y, center.z),
			Vector::new(center.x, center.y, self.min.z),
			Vector::new(self.max.x, center.y, center.z),
			Vector::new(center.x, self.max.y, center.z),
			Vector::new(center.x, center.y, self.max.z),
		]
	}

	/// Get the center of the `BBox`.
	pub fn center(&self) -> Vector {
		Vector::new(
			(self.min.x + self.max.x) / 2.0,
			(self.min.y + self.max.y) / 2.0,
			(self.min.z + self.max.z) / 2.0,
		)
	}
}
