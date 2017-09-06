// Aldaron's Memory Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/heap_mem.rs

use core::ptr;

use Void;

/// Memory on the heap - managed by Rust.
#[repr(C)]
pub struct HeapMem<T>(*mut T);

// TODO: Only malloc, realloc and free if have libc, alternatives when don't.
impl<T> HeapMem<T> {
	/// Reserve `size` bytes of memory and return it's address (a pointer).
	#[inline(always)]
	pub fn new(size: usize) -> HeapMem<T> {
		if size == 0 {
			return HeapMem(ptr::null_mut());
		}

		extern "C" {
			fn malloc(n: usize) -> *mut Void;
		}

		unsafe { HeapMem(malloc(size) as *mut _) }
	}

	/// Allocates memory on the heap and then places `x` into it.
	/// Doesn't actually allocate if `T` is zero-sized.
	#[inline(always)]
	pub fn from(x: T) -> HeapMem<T> {
		let mut heap = HeapMem::new(1);

		*heap = x;

		heap
	}

	/// Resize memory at Equivalent to a call to `realloc()`
	#[inline(always)]
	pub fn resize(&mut self, size: usize) -> () {
		if size == 0 {
			panic!("Error: Can't call resize() when size == 0.");
		}

		extern "C" {
			fn realloc(p: *mut Void, n: usize) -> *mut Void;
		}

		self.0 = unsafe { realloc(self.0 as *mut _, size) } as *mut _;
	}

	/// Convert `HeapMem` to `*const T`.
	#[inline(always)]
	pub fn as_ptr(&self) -> *const T {
		self.0 as *const _
	}

	/// Convert `HeapMem` to `*mut _`.
	#[inline(always)]
	pub fn as_mut_ptr(&self) -> *mut T {
		self.0 as *mut _
	}

	/// Convert `HeapMem` to `&mut [T]`.
	#[inline(always)]
	pub unsafe fn as_slice(&mut self, len: usize) -> &mut [T] {
		::core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _,
			len)
	}
}

impl<T> Drop for HeapMem<T> {
	#[inline(always)]
	fn drop(&mut self) -> () {
		extern "C" {
			fn free(p: *mut Void) -> ();
		}

		unsafe { free(self.0 as *mut _) };
	}
}


impl<T> ::core::ops::Deref for HeapMem<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

impl<T> ::core::ops::DerefMut for HeapMem<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.0 }
	}
}
