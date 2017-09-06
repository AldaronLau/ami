// Aldaron's Memory Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/mem_addr.rs

/// Pointer to memory managed by C.
#[repr(C)]
pub struct MemAddr<T>(*mut T);

impl<T> MemAddr<T> {
	/// Create `MemAddr` from a raw pointer.
	#[inline(always)]
	pub unsafe fn new(pointer: *mut T) -> MemAddr<T> {
		MemAddr(pointer)
	}

	/// Get a raw pointer from `MemAddr`.
	#[inline(always)]
	pub fn as_ptr<U>(&self) -> *const U {
		self.0 as *const _
	}

	/// Get an unsafe mutable raw pointer from `MemAddr`.
	#[inline(always)]
	pub fn as_mut_ptr<U>(&mut self) -> *mut U {
		self.0 as *mut _
	}

	/// Get a slice from `MemAddr`.
	#[inline(always)]
	pub unsafe fn as_slice<U>(&mut self, len: usize) -> &mut [U] {
		::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
	}
}
