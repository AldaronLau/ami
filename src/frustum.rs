// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::fmt;

use Vec3;
use BCube;
use BBox;
// use math::Plane;

#[derive(Clone, Copy, PartialEq)]
/// Single-precision frustum
pub struct Frustum {
	/// The center of the frustum
	pub center: Vec3,
	/// The radius of the frustum
	pub radius: f32,
	/// The fov in x
	pub wfov: f32,
	/// the fov in y
	pub hfov: f32,
	/// how much rotated from facing "straight forward" in x
	pub xrot: f32,
	/// how much rotated from facing "straight forward" in y
	pub yrot: f32,
}

impl fmt::Debug for Frustum {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(radius: {:?})", self.radius)
	}
}

impl Frustum {
	/// Create a new viewing frustum.
	///
	/// * `center` - The center of the frustum cone.
	/// * `radius` - How far can you see?
	/// * `xrot` - Direction facing on x axis (radians).
	/// * `yrot` - Direction facing on y axis (radians).
	/// * `wfov` - The fov on the X axis (radians).
	/// * `hfov` - The fov on the Y axis (radians).
	pub fn new(center: Vec3, radius: f32, xrot: f32, yrot: f32,
		wfov: f32, hfov: f32) -> Frustum
	{
/*		let xmax = far / (wfov / 2.0).tan();
		let ymax = far / (hfov / 2.0).tan();

		let rightfar = Vec3::new(xmax, 0.0, far);
		let leftfar = Vec3::new(-xmax, 0.0, far);
		let topfar = Vec3::new(0.0, -ymax, far);
		let bottomfar = Vec3::new(0.0, ymax, far);
//		let camera = Vec3::new(0.0, 0.0, -ar);

		let wdist = ((::std::f32::consts::PI - wfov) / 2.0).sin() * -xmax;
		let hdist = ((::std::f32::consts::PI - hfov) / 2.0).sin() * -ymax;

		let top = Plane::new(bottomfar, hdist);
		let bottom = Plane::new(topfar, hdist);
		let right = Plane::new(leftfar, wdist);
		let left = Plane::new(rightfar, wdist);
		let near = Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0);
		let far = Plane::new(Vec3::new(0.0, 0.0, -1.0), -far);

		Frustum { near, far, top, bottom, right, left }*/

		Frustum { center, radius, xrot, yrot, wfov, hfov }
	}

	/// 
	pub fn collide_bbox(&self, bbox: BBox) -> bool {
		for i in bbox.all_points().iter() {
			if (*i - self.center).length() <= self.radius {
				return true;
			}
		}

		false
	}

	/// If viewing frustum collides with the bounding box.
	pub fn collide_bcube(&self, bcube: BCube) -> bool {
		for i in bcube.all_points().iter() {
			if (*i - self.center).length() <= self.radius {
				return true;
			}
		}

		false

/*		let top = self.top;
		let bottom = self.bottom;
		let right = self.right;
		let left = self.left;
		let near = self.near;
		let far = self.far;*/

/*		let planes = [self.top, self.bottom, self.right, self.left,
			self.near, self.far];

		for plane in planes.iter() {
			let (a, b) = bcube.pn_pair_from_normal(plane.facing);

			if !plane.isdistpos_point(a) && !plane.isdistpos_point(b) {
				return false;
			}
		}*/

/*		// All 6 planes must have a point within their area.
		top.isdistpos_bcube(bcube) && bottom.isdistpos_bcube(bcube) &&
			right.isdistpos_bcube(bcube) && left.isdistpos_bcube(bcube)
			&& near.isdistpos_bcube(bcube) && far.isdistpos_bcube(bcube)*/
	}

	/// If viewing frustum collides with a point.
	pub fn collide_point(&self, point: Vec3) -> bool {
		(point - self.center).length() <= self.radius

/*		self.near.isdistpos_point(point)
			&& self.far.isdistpos_point(point)
			&& self.left.isdistpos_point(point)
			&& self.right.isdistpos_point(point)
			&& self.top.isdistpos_point(point)
			&& self.bottom.isdistpos_point(point)*/
	}
}
