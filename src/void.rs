// void.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use core::ptr;
use core::marker::PhantomData;

#[cfg(target_pointer_width = "32")]
type NativePtr = u32;

#[cfg(target_pointer_width = "64")]
type NativePtr = u64;

/// Void type for C's `void *`.  Use as `*mut Void`.
#[repr(u8)]
pub enum Void {
	#[doc(hidden)]
	__DontUseMe,
	#[doc(hidden)]
	__DontUseMe2,
}

/// An unsafe wrapper around a Void pointer.
pub struct UnsafeData(NativePtr);

/// A safe wrapper around a void pointer.
pub struct RawData(UnsafeData);

/// A safe wrapper around a `T` pointer.
pub struct HeapData<T>(RawData, PhantomData<T>);

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
	pub fn as_ptr<T>(&self) -> *const T {
		unsafe {
			RawData::transmute(self.0)
		}
	}

	/// Get an unsafe mutable raw pointer from UnsafeData.
	#[inline(always)]
	pub fn as_mut_ptr<T>(&mut self) -> *mut T {
		unsafe {
			RawData::transmute(self.0)
		}
	}

	/// Get a slice from UnsafeData.
	#[inline(always)]
	pub unsafe fn as_slice<T>(&mut self, len: usize) -> &mut [T] {
		::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
	}
}

// TODO: Only malloc, realloc and free if have libc, alternatives when don't.
impl Void {
	/// Equivalent to a call to `malloc()`
	#[inline(always)]
	pub unsafe fn new(size: usize) -> *mut Void {
		extern "C" {
			fn malloc(n: usize) -> *mut Void;
		}

		malloc(size)
	}

	/// Equivalent to a call to `realloc()`
	#[inline(always)]
	pub unsafe fn resize(pointer: &mut *mut Void, size: usize) -> () {
		if size == 0 {
			panic!("Error: Can't call resize() when size == 0.");
		}

		extern "C" {
			fn realloc(pointer: *mut Void, n: usize) -> *mut Void;
		}

		*pointer = realloc(*pointer, size);
	}

	/// Equivalent to a call to `free()`
	#[inline(always)]
	pub unsafe fn drop(pointer: *mut Void) -> () {
		extern "C" {
			fn free(pointer: *mut Void) -> ();
		}

		free(pointer)
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
		Void::resize(&mut self.as_mut_ptr(), size);
	}

	/// Copy bits ( raw data ) from T to U.  Make sure types are same size.
	/// This function is unsafe because it does no checking.
	#[inline(always)]
	pub unsafe fn transmute<T,U>(from: T) -> U {
		ptr::read(&from as *const _ as *const U)
	}

	/// Convert the RawData into a raw pointer.
	#[inline(always)]
	pub fn as_ptr<T>(&self) -> *const T {
		self.0.as_ptr()
	}

	/// Convert the RawData into an unsafe mutable raw pointer.
	#[inline(always)]
	pub fn as_mut_ptr<T>(&mut self) -> *mut T {
		self.0.as_mut_ptr()
	}
}

impl Drop for RawData {
	#[inline(always)]
	fn drop(&mut self) {
		unsafe { Void::drop(self.as_mut_ptr()); }
	}
}

impl<T> HeapData<T> {
	/// Allocate space for `size` elements on the heap.  The memory is
	/// automatically free'd.
	#[inline(always)]
	pub fn new(size: usize) -> HeapData<T> {
		HeapData(RawData::new(size), PhantomData)
	}

	/// Allocates memory on the heap and then places `x` into it.
	/// Doesn't actually allocate if `T` is zero-sized.
	#[inline(always)]
	pub fn from(x: T) -> HeapData<T> {
		let mut heap = HeapData::new(1);

		*heap = x;

		heap
	}
}

impl<T> ::core::ops::Deref for HeapData<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.as_ptr() }
	}
}

impl<T> ::core::ops::DerefMut for HeapData<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.0.as_mut_ptr() }
	}
}

//	#[inline(always)]
//	pub fn 	
// }
