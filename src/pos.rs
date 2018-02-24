// Aldaron's Memory Interface
// Copyright (c) 2017  Douglas P Lau
// Copyright (c) 2017-2018, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// pos.rs

use Vec3;

/// Pos trait allows point lookup by handle
pub trait Pos {
/*	fn add(&mut self, p: Vec3<f32>) -> u32;
	fn wrt(&mut self, hnd: u32, p: Vec3<f32>);
	fn len(&self) -> usize;*/
	/// Position as float
	fn posf(&self/*, hnd: u32*/) -> Vec3<f32>;
	/// Position as int
	fn posi(&self/*, hnd: u32*/) -> Vec3<i32>;
}
