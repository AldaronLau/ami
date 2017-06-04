// void_pointer.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! This module is for repurposing memory returned from or sent to FFI.

/// A type that represents a void* in C.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct VoidPointer {
	#[cfg(target_pointer_width = "32")]
	native: u32,
	#[cfg(target_pointer_width = "64")]
	native: u64,
}

/// Equivalent of NULL in C.
pub const NULL : VoidPointer = VoidPointer { native: 0 };

/// A trait used for casting the void pointer to other pointer types.
pub trait VoidPointerCast<T> {
	/// Cast a VoidPointer to a native pointer of any type.
	#[inline(always)]
	fn cast(&self) -> *mut T;
	/// Cast a native pointer of any type to a VoidPointer.
	#[inline(always)]
	fn from(pointer: *mut T) -> VoidPointer;
}

impl<T> VoidPointerCast<T> for VoidPointer {
	#[inline(always)]
	fn from(pointer: *mut T) -> VoidPointer {
		unsafe {
			*(&pointer as *const *mut _ as *const _)
		}
	}

	#[inline(always)]
	fn cast(&self) -> *mut T {
		unsafe {
			*(&self as *const _ as *const *mut _)
		}
	}
}
