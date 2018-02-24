// Aldaron's Memory Interface
// Copyright (c) 2017  Douglas P Lau
// Copyright (c) 2017-2018, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// vec4.rs

use std::fmt;
use std::cmp;

/// 4-dimensional vector
#[derive(Clone, Copy, PartialEq)]
pub struct Vec4<T: Copy + Clone> {
	/// X coordinate
	pub x: T,
	/// Y coordinate
	pub y: T,
	/// Z coordinate
	pub z: T,
	/// W coordinate
	pub w: T,
}

impl<T> fmt::Debug for Vec4<T> where T: fmt::Debug + Copy + Clone {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"({:?},{:?},{:?},{:?})",self.x,self.y,self.z,self.w)
	}
}

#[allow(unused)]
impl<T> Vec4<T> where T: Copy + Clone {
	/// Create a new Vec4
	pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
		Vec4 { x, y, z, w }
	}

	/// Find the minimum ordinal value
	pub(crate) fn min_p(self) -> T where T: cmp::Ord {
		self.x.min(self.y).min(self.z).min(self.w)
	}

	/// Find the maximum ordinal value
	pub(crate) fn max_p(self) -> T where T: cmp::Ord {
		self.x.max(self.y).max(self.z).max(self.w)
	}
}
