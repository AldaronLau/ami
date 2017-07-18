// vec.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! A growable array on the heap.

use void_pointer::*;
use size_of::*;

pub struct Vec<T> {
	ptr: TypePointer<T>,
	cap: usize,
	len: usize,
}

impl<T> Vec<T> {
	/// Create an empty `Vec<T>`.
	#[inline(always)]
	pub fn new() -> Vec<T> {
		let ptr = NULL.as_type::<T>();
		let cap = 0;
		let len = 0;

		Vec { ptr, cap, len }
	}

	/// Append an element at the end of the `Vec<T>`.
	#[inline(always)]
	pub fn push(&mut self, elem: T) -> () {
		// If it needs to grow, re-allocate.
		if self.grow() {
			self.realloc();
		}

		// Initialize the uninitialized.
		self.ptr[self.len] = elem;

		// Length has increased by one.
		self.len += 1;
	}

	/// Remove the last element of the `Vec<T>` and return it, or `None` if
	/// the `Vec<T>` is empty.
	#[inline(always)]
	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			return None;
		}

		// Length has decreased by one.
		self.len -= 1;

		// This is safe because we're moving the value out of the vector
		// The copied value is out of bounds, so it's a move.
		Some(unsafe { self.ptr.copy_index(self.len) })
	}

	// This will add capacity if len > cap
	#[inline(always)]
	fn grow(&mut self) -> bool {
		// Don't do anything if cap is fine.
		if self.len < self.cap || size_of::<T>() == 0 {
			false
		} else {
			// If cap is 0, make it one so that the next step works.
			if self.cap == 0 {
				self.cap = 1;
			}

			// cap is a power of two.
			self.cap *= 2;

			// Resize was needed.
			true
		}
	}

	// Realloc ptr from capacity.
	#[inline(always)]
	fn realloc(&mut self) {
		let mut ptr = self.ptr.as_void();
		let bytes = self.cap * size_of::<T>();

		self.ptr = unsafe {
			::heap::resize(&mut ptr.as_ptr(), bytes);
			ptr.as_type::<T>()
		};
	}
}

impl<T> Drop for Vec<T> {
	#[inline(always)]
	fn drop(&mut self) {
		if self.cap != 0 {
			unsafe { ::heap::drop(*(self.ptr.as_void().as_ptr())) };
		}
	}
}

impl<T> ::core::fmt::Display for Vec<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		write!(f, "{{")?;
		write!(f, "}}")
	}
}

impl<T> ::core::ops::Deref for Vec<T> {
	type Target = [T];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe {
			::core::slice::from_raw_parts(self.ptr.cast(), self.len)
		}
	}
}

impl<T> ::core::ops::DerefMut for Vec<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		let ptr = self.ptr.cast();

		unsafe {
			::core::slice::from_raw_parts_mut(ptr, self.len)
		}
	}
}
