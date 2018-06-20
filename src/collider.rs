// "ami" - Aldaron's Memory Interface
//
// Copyright Douglas P. Lau 2017.
// Copyright Jeron A. Lau 2017 - 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use BBox;

/// `Collider` has bounding box and id.
pub trait Collider {
	/// Get the `BBox` for this collider.
	fn bbox(&self) -> BBox;
}
