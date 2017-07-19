// void.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use core::ptr;

/// Void type for C's `void *`.  Use as `*mut Void`.
#[repr(u8)]
pub enum Void {
	#[doc(hidden)]
	__DontUseMe,
	#[doc(hidden)]
	__DontUseMe2,
}

/// An unsafe wrapper around a void pointer.
pub struct UnsafeData(
	#[cfg(target_pointer_width = "32")]
	u32,
	#[cfg(target_pointer_width = "64")]
	u64,
);

/// A safe wrapper around a void pointer.
pub struct RawData(UnsafeData);

/// Represents a NULL pointer.  To use with ffi: `NULL.as_ptr()`
pub const NULL : UnsafeData = UnsafeData(0);

impl UnsafeData {
	/// Create UnsafeData from a raw pointer.
	#[inline(always)]
	pub unsafe fn new<T>(pointer: *mut T) -> UnsafeData {
		UnsafeData(RawData::transmute(pointer))
	}

	/// Get a raw pointer from UnsafeData.
	#[inline(always)]
	pub unsafe fn as_ptr<T>(&self) -> *const T {
		RawData::transmute(self.0)
	}

	/// Get a raw pointer from UnsafeData.
	#[inline(always)]
	pub unsafe fn as_mut_ptr<T>(&mut self) -> *mut T {
		RawData::transmute(self.0)
	}

	/// Get a slice from UnsafeData.
	#[inline(always)]
	pub unsafe fn as_slice<T>(&mut self, len: usize) -> &mut [T] {
		::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
	}
}

impl Void {
	/// Equivalent to a call to `malloc()`
	#[inline(always)]
	pub unsafe fn new(size: usize) -> *mut Void {
		::heap_ffi::allocate(size)
	}

	/// Equivalent to a call to `realloc()`
	#[inline(always)]
	pub unsafe fn resize(pointer: &mut *mut Void, size: usize) -> () {
		if size == 0 {
			panic!("Error: Can't call resize() when size == 0.");
		}
		::heap_ffi::resize(pointer, size)
	}

	/// Equivalent to a call to `free()`
	#[inline(always)]
	pub unsafe fn drop(pointer: *mut Void) -> () {
		::heap_ffi::drop(pointer)
	}
}

impl RawData {
	/// Allocate `size` bytes.  The memory is automatically free'd.
	#[inline(always)]
	pub fn new(size: usize) -> RawData {
		RawData( unsafe { UnsafeData::new(Void::new(size)) } )
	}

	/// Resize to `size` bytes.  This is unsafe because any new memory is
	/// uninitialized.
	#[inline(always)]
	pub unsafe fn resize(&mut self, size: usize) -> () {
		Void::resize(&mut self.as_ptr(), size);
	}

	/// Copy bits ( raw data ) from T to U.  Make sure types are same size.
	/// This function is unsafe because it does no checking.
	#[inline(always)]
	pub unsafe fn transmute<T,U>(from: T) -> U {
		ptr::read(&from as *const _ as *const U)
	}

	/// Convert the RawData into a raw pointer.  This is unsafe because
	/// RawData isn't consumed, so there could be refrences to the same data
	/// that iterpret it as different types.
	#[inline(always)]
	pub unsafe fn as_ptr<T>(&mut self) -> *mut T {
		self.0.as_mut_ptr()
	}
}

impl Drop for RawData {
	#[inline(always)]
	fn drop(&mut self) {
		unsafe { Void::drop(self.as_ptr()); }
	}
}
