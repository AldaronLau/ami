// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017  Douglas P. Lau
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use BBox;

/// `Collider` has bounding box and id.
pub trait Collider {
	/// Get the `BBox` for this collider.
	fn bbox(&self) -> BBox;
}
