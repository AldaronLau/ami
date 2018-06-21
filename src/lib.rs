// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! # [Aldaron's Memory Interface](https://crates.io/crates/ami)
//! Aldaron's Memory Interface provides data structures and casting macros.
//! 
//! ## Features
//! **ami**'s current features:
//! * Casting pointers with the `cast!()` and `cast_mut!()` macros
//! * Provide geometrical data structs, an do math with them
//! * Automatic-size-adjusting octree

#![warn(missing_docs)]
#![doc(
	html_logo_url = "https://plopgrizzly.com/ami/icon.png",
	html_favicon_url = "https://plopgrizzly.com/ami/icon.png",
	html_root_url = "http://plopgrizzly.com/ami/"
)]

#[macro_use]
extern crate euler;

pub use euler::*;

mod bbox;
mod bcube;
mod frustum;
mod octree;
mod plane;
mod collider;
mod parent;

pub use bcube::*;
pub use bbox::*;
pub use frustum::*;
pub use octree::{ Octree, Id };
pub use plane::*;
pub use collider::*;
pub use parent::*;

/// Cast a constant pointer to another type.
#[macro_export] macro_rules! cast {
	($a:expr) => {
		$a as *const _ as *const _
	}
}

/// Cast a mutable pointer to another type.
#[macro_export] macro_rules! cast_mut {
	($a:expr) => {
		$a as *mut _ as *mut _
	}
}
