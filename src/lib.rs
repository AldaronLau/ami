// lib.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! Aldaron's Memory Interface (A.M.I.) is a Rust library for manipulating
//! memory.

#![doc(html_logo_url = "http://at.plopgrizzly.tech/ami/icon.png",
       html_favicon_url = "http://at.plopgrizzly.tech/ami/icon.png",
       html_root_url = "http://at.plopgrizzly.tech/ami/")]

#![no_std] // No Standard Library.

// TODO: Only if have libc, alternatives when don't.
pub mod heap {
	//! Simple heap functions.

	extern crate libc;

	pub use self::libc::c_void as void;

	/// Allocate memory on the heap.
	#[inline(always)]
	pub unsafe fn allocate(n: usize) -> *mut void {
		libc::malloc(n)
	}

	/// Resize memory on the heap.
	#[inline(always)]
	pub unsafe fn resize(pointer: &mut *mut void, n: usize) -> () {
		*pointer = libc::realloc(*pointer, n);
	}

	/// Drop memory on the heap.
	#[inline(always)]
	pub unsafe fn drop(pointer: *mut void) -> () {
		libc::free(pointer);
	}
}

/*pub mod heap {
	
}*/

pub mod void_pointer;
pub mod repurpose;
pub mod size_of;
pub mod boxed;
pub mod vec;

/** Void type for C's `void *`.  Use as `*mut void`. */
pub use heap::void;