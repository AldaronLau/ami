// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! # [Aldaron's Memory Interface](https://crates.io/crates/ami)
//! Aldaron's Memory Interface provides useful data structures that are not in
//! the standard library.
//! 
//! ## Features
//! **ami**'s current features:
//! * Provide geometrical data structs, and do math with them
//! * Automatic-size-adjusting octree

#![warn(missing_docs)]
#![doc(
	html_logo_url = "https://plopgrizzly.com/ami/icon.png",
	html_favicon_url = "https://plopgrizzly.com/ami/icon.png",
	html_root_url = "http://plopgrizzly.com/ami/"
)]

#[macro_use]
extern crate euler;

pub mod macros;

mod bbox;
mod bcube;
mod frustum;
mod octree;
mod plane;
mod collider;

pub use bcube::*;
pub use bbox::*;
pub use frustum::*;
pub use octree::{ Octree, Id };
pub use plane::*;
pub use collider::*;

pub use euler::{ Mat2, Mat3, Mat4, Quat, Trs, Vec2, Vec3, Vec4 };
