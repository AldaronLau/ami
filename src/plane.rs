// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::fmt;

use Vector;
use BCube;

#[derive(Clone, Copy, PartialEq)]
/// Single-precision plane
pub struct Plane {
	/// A normalized directional vector for the direction the plane faces.
	pub facing: Vector,
	/// The offset of the plane from the origin in the direction of `facing`
	pub offset: f32,
}

impl fmt::Debug for Plane {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Offset: {:?} Facing: {:?}", self.offset, self.facing)
	}
}

impl Plane {
	/// Create a new plane from directional vector, and offset from origin.
	pub fn new(dir: Vector, ofs: f32) -> Plane {
		Plane { facing: dir.normalize(), offset: ofs }
	}

	/// Returns true if distance from Plane to point is positive.
	pub fn isdistpos_point(&self, p: Vector) -> bool {
		(self.facing.x * (p.x - (self.facing.x * self.offset)))
			+ (self.facing.y * (p.y - (self.facing.y * self.offset)))
			+ (self.facing.z * (p.z - (self.facing.z * self.offset)))
			 >= 0.0
	}

	/// Returns true if distance from Plane to BCube is positive
	pub fn isdistpos_bcube(&self, bcube: BCube) -> bool {
		let (_, b) = bcube.pn_pair_from_normal(self.facing);

/*		let pos_side = self.facing.dot(b) + self.offset;
//		let pos_side = (self.facing.x * a.x)+(self.facing.y * a.y)+(self.facing.z * a.z)+self.offset;
		if pos_side > 0.0 {
			//box is completely on positive side of plane
			return true;
		}*/
		let neg_side = self.facing.dot(b.into()) - self.offset;
//		let neg_side = (self.facing.x * b.x)+(self.facing.y * b.y)+(self.facing.z * b.z)+self.offset;
		if neg_side < 0.0 {
			//box is completely on negative side of plane
			return false;
		}

		true

/*		let points = bcube.all_points();

		for point in points.iter() {
			// If point within, it's good!
			if self.isdistpos_point(*point) {
				return true;
			}
		}*/

//		let (a, b) = bcube.pn_pair_from_normal(self.facing);

		// If the extremes of the BCube is within the bounded area
//		self.isdistpos_point(a) || !self.isdistpos_point(b)

//		false
	}
}

/*#[test]
fn test_plane_distpos() {
	let t = ::Transform::new()
		.rotate(-10.0, 20.0, -5.0)
		.translate(500.0, -100.0, -115.0)
		.rotate(1.0, 2.0, 0.3);

	let a = Plane::new(t * Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0).transform_dir(t));
	let b = Plane::new(t * Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 0.0, -1.0).transform_dir(t));
	let c = Plane::new(t * Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0).transform_dir(t));

	assert!(a.isdistpos_point(t * Vector::new(-12.0, 2.0, 0.0)) == true);
	assert!(a.isdistpos_point(t * Vector::new(-12.0, 0.0, 0.0)) == false);
	assert!(b.isdistpos_point(t * Vector::new(15.0, 0.0, -12.0)) == true);
	assert!(b.isdistpos_point(t * Vector::new(15.0, 0.0, 12.0)) == false);
	assert!(c.isdistpos_point(t * Vector::new(5.0, -10.0, 0.5)) == true);
	assert!(c.isdistpos_point(t * Vector::new(-5.0, -10.0, -0.5)) == false);
}*/

/*#[test]
fn test_bcube_in_plane() {
	let t = ::Transform::new()
		.rotate(-10.0, 20.0, -5.0)
		.translate(500.0, -100.0, -115.0)
		.rotate(1.0, 2.0, 0.3);

	// Plane is behind box	
	let a = Plane::new(Vector::new(0.0, 0.0, 1.0)/*.transform_dir(t)*/, -2.5);
	// Plane intersects box
	let b = Plane::new(Vector::new(0.0, 0.0, 1.0)/*.transform_dir(t)*/, 0.0);
	// Plane is in front of box, plz cull
	let c = Plane::new(Vector::new(0.0, 0.0, 1.0)/*.transform_dir(t)*/, 2.5);
	// Plane intersects box at 45° angle
	let d = Plane::new(Vector::new(0.0, 0.0, 1.0).transform_dir(
		::Transform::new()
			.rotate(0.0, -0.5, 0.0)), 0.0);
	// Plane intersects box at 135° angle
	let e = Plane::new(Vector::new(0.0, 0.0, 1.0).transform_dir(
		::Transform::new()
			.rotate(0.0, -1.5, 0.0)), 0.0);
	// Plane is behind box from 45° angle.
	let f = Plane::new(Vector::new(0.0, 0.0, 1.0).transform_dir(
		::Transform::new()
			.rotate(0.0, 0.25, 0.0)), -2.5);
	// Plane is behind box from 135° angle.	
	let g = Plane::new(Vector::new(0.0, 0.0, 1.0).transform_dir(
		::Transform::new()
			.rotate(0.0, 0.75, 0.0)), -2.5);
	// Plane is behind box from 225° angle.
	let h = Plane::new(Vector::new(0.0, 0.0, 1.0).transform_dir(
		::Transform::new()
			.rotate(0.0, 1.25, 0.0)), -2.5);
	// Plane is behind box from 315° angle.
	let i = Plane::new(Vector::new(0.0, 0.0, 1.0).transform_dir(
		::Transform::new()
			.rotate(0.0, 1.75, 0.0)), -2.5);

	assert!(a.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(b.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(c.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == false);
	assert!(d.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(e.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(f.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(g.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(h.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);
	assert!(i.isdistpos_bcube(BCube::new(Vector::new(0, 0.0, 0.0))) == true);

	assert!(a.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(b.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(c.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == false);
	assert!(d.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(e.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(f.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(g.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(h.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
	assert!(i.isdistpos_bcube(BCube::new(Vector::new(0, -2.0, 0.0))) == true);
}*/
