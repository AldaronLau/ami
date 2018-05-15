// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//
//! Aldaron's Memory Interface provides data structures and casting macros.

#![warn(missing_docs)]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/plopgrizzly\
		/ami/master/res/icon.png",
	html_favicon_url = "https://raw.githubusercontent.com/plopgrizzly\
		/ami/master/res/symbol.svg",
	html_root_url = "http://plopgrizzly.com/ami/"
)]

mod mat4;
mod bbox;
mod frustum;
mod octree;
mod plane;
mod pos;
mod vec2;
mod vec3;
mod vec4;
mod parent;

pub use mat4::*;
pub use bbox::*;
pub use frustum::*;
pub use octree::*;
pub use plane::*;
pub use pos::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
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
