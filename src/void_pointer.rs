// void_pointer.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! This module is for repurposing memory returned from or sent to FFI.

use core::ops::{Add, Sub, Not, BitAnd, BitOr, BitXor, Shr, Shl};

#[cfg(target_pointer_width = "32")]
type NativePtr = u32;

#[cfg(target_pointer_width = "64")]
type NativePtr = u64;

/// A type that represents a void* in C.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct VoidPointer {
	native: NativePtr,
}

/// Equivalent of NULL in C.
pub const NULL : VoidPointer = VoidPointer { native: 0 };

impl VoidPointer {
	/// Return the pointer as an integer.
	#[inline(always)]
	pub fn as_int(&self) -> NativePtr {
		self.native
	}
}
/*
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
}*/

impl Add<u32> for VoidPointer {
	type Output = VoidPointer;

	fn add(self, other: u32) -> VoidPointer {
		VoidPointer {
			native: self.native + (other as NativePtr)
		}
	}
}

impl Sub<u32> for VoidPointer {
	type Output = VoidPointer;

	fn sub(self, other: u32) -> VoidPointer {
		VoidPointer {
			native: self.native - (other as NativePtr)
		}
	}
}

impl BitAnd<u32> for VoidPointer {
	type Output = VoidPointer;
	
	fn bitand(self, other: u32) -> VoidPointer {
		VoidPointer {
			native: (self.native as u32 & other) as NativePtr
		}
	}
}

impl BitOr<u32> for VoidPointer {
	type Output = VoidPointer;
	
	fn bitor(self, other: u32) -> VoidPointer {
		VoidPointer {
			native: (self.native as u32 | other) as NativePtr
		}
	}
}

impl BitXor<u32> for VoidPointer {
	type Output = VoidPointer;
	
	fn bitxor(self, other: u32) -> VoidPointer {
		VoidPointer {
			native: (self.native as u32 ^ other) as NativePtr
		}
	}
}

impl Shr<usize> for VoidPointer {
	type Output = VoidPointer;
	
	fn shr(self, rhs: usize) -> VoidPointer {
		VoidPointer {
			native: self.native >> rhs
		}
	}
}

impl Shl<usize> for VoidPointer {
	type Output = VoidPointer;
	
	fn shl(self, rhs: usize) -> VoidPointer {
		VoidPointer {
			native: self.native << rhs
		}
	}
}

impl Not for VoidPointer {
	type Output = VoidPointer;
	
	fn not(self) -> VoidPointer {
		VoidPointer {
			native: !self.native
		}
	}
}