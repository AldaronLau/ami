// Copyright Jeron A. Lau 2017-2018.
// Copyright Douglas Lau 2017
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
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
//!
//! ## Getting started
//! ```
//! extern crate ami;
//! use ami::*;
//! ```

#![warn(missing_docs)]
#![doc(
	html_logo_url = "https://plopgrizzly.com/ami/icon.png",
	html_favicon_url = "https://plopgrizzly.com/ami/icon.png",
	html_root_url = "http://plopgrizzly.com/ami/"
)]

extern crate cgmath;

#[macro_use]
mod macros;
mod bbox;
mod bcube;
mod frustum;
mod octree;
mod plane;
mod collider;
mod vector;
mod matrix;
mod rotation;

pub use bcube::BCube;
pub use bbox::BBox;
pub use frustum::Frustum;
pub use octree::{Octree, Id};
pub use plane::Plane;
pub use collider::Collider;
pub use vector::Vector;
pub use matrix::Matrix;
pub use rotation::Rotation;
