// boxed.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use heap;
use size_of;
use void_pointer::*;

pub struct Box<T>(TypePointer<T>);

impl<T> Box<T> {
	/// Allocates memory on the heap and then places `x` into it.
	/// Doesn't actually allocate if `T` is zero-sized.
	#[inline(always)]
	pub fn from(x: T) -> Box<T> {
		let heap = heap::alloc(NULL, size_of::size_of::<T>());
		let mut tptr = heap.as_type();

		*tptr = x;

		Box(tptr)
	}

	/// Get an immutable reference to the boxed type.
	#[inline(always)]
	pub fn as_ref(&self) -> &T {
		unsafe { &*self.0.cast() }
	}
}

impl<T> Drop for Box<T> {
	#[inline(always)]
	fn drop(&mut self) {
		heap::alloc(self.0.as_void(), 0);
	}
}
