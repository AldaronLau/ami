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
pub mod heap_ffi {
	//! Simple heap functions.

	use super::Void;

	/// Allocate memory on the heap.
	#[inline(always)]
	pub unsafe fn allocate(n: usize) -> *mut Void {
		extern "C" {
			fn malloc(n: usize) -> *mut Void;
		}

		malloc(n)
	}

	/// Resize memory on the heap.
	#[inline(always)]
	pub unsafe fn resize(pointer: &mut *mut Void, n: usize) -> () {
		extern "C" {
			fn realloc(pointer: *mut Void, n: usize) -> *mut Void;
		}

		*pointer = realloc(*pointer, n);
	}

	/// Drop memory on the heap.
	#[inline(always)]
	pub unsafe fn drop(pointer: *mut Void) -> () {
		extern "C" {
			fn free(pointer: *mut Void) -> ();
		}

		free(pointer);
	}
}

/*pub mod heap {
	
}*/

pub mod void_pointer;
pub mod repurpose;
pub mod size_of;
pub mod boxed;
pub mod vec;

mod void;

pub use void::*;
