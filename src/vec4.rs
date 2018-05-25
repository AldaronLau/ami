// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017  Douglas P. Lau
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::fmt;

/// 4-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec4 {
	/// X coordinate
	pub x: f32,
	/// Y coordinate
	pub y: f32,
	/// Z coordinate
	pub z: f32,
	/// W coordinate
	pub w: f32,
}

impl fmt::Debug for Vec4 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
	}
}

#[allow(unused)]
impl Vec4 {
	/// Create a new Vec4
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
		Vec4 { x, y, z, w }
	}

	/// Find the minimum ordinal value
	pub(crate) fn min_p(self) -> f32 {
		self.x.min(self.y).min(self.z).min(self.w)
	}

	/// Find the maximum ordinal value
	pub(crate) fn max_p(self) -> f32 {
		self.x.max(self.y).max(self.z).max(self.w)
	}
}
