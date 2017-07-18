// void_pointer.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! This module is for repurposing memory returned from or sent to FFI.

use core::ops::{ Add, Sub, Not, Shr, Shl, Deref, DerefMut, Index, IndexMut };
use core::marker::PhantomData;
use core::fmt::{ Display, Result, Formatter };
use core::ptr;

use repurpose::*;

#[cfg(target_pointer_width = "32")]
type NativePtr = u32;

#[cfg(target_pointer_width = "64")]
type NativePtr = u64;

/// A type that represents a void* in C.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct VoidPointer(NativePtr);

/// A type that represents a `T`* in C.
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct TypePointer<T> where T: ?Sized {
	native: NativePtr,
	marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for TypePointer<T> {}
unsafe impl<T: Sync> Sync for TypePointer<T> {}

/// Equivalent of NULL in C.
pub const NULL : VoidPointer = VoidPointer(0);

impl VoidPointer {
	/// Allocate `n` bytes on the stack.  This is unsafe because it still
	/// has to be free'd.
	#[inline(always)]
	pub unsafe fn new(n: usize) -> VoidPointer {
		VoidPointer(*(repurpose(&mut ::heap::allocate(n))))
	}

	/// Return the pointer as an integer.
	#[inline(always)]
	pub fn as_int(&self) -> NativePtr {
		self.0
	}

	/// Cast to a TypePointer for indexing or dereferencing.
	#[inline(always)]
	pub fn as_type<T>(&self) -> TypePointer<T> {
		TypePointer {
			native: self.0,
			marker: PhantomData
		}
	}
	
	#[inline(always)]
	pub fn as_ptr(&mut self) -> &mut *mut ::void {
		::repurpose::repurpose(&mut self.0)
	}
}

impl<T> TypePointer<T> {
	/// Inverse of `VoidPointer::as_type<T>()`
	#[inline(always)]
	pub fn as_void(&self) -> VoidPointer {
		VoidPointer(self.native)
	}

	#[inline(always)]
	pub unsafe fn copy_index(&self, index: usize) -> T {
		ptr::read(&self[index])
	}

	#[inline(always)]
	pub fn swap_index(&mut self, index: usize, value: T) -> T {
		let read = unsafe { self.copy_index(index) };

		self[index] = value;

		read
	}
}

/// A trait used for casting the void pointer to other pointer types.
pub trait PointerCast<T> {
	/// Cast a VoidPointer to a native pointer of any type.
	#[inline(always)]
	fn cast(&self) -> *mut T;
	/// Cast a native pointer of any type to a VoidPointer.
	#[inline(always)]
	fn from(pointer: *mut T) -> VoidPointer;
}

impl<T> PointerCast<T> for VoidPointer {
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

impl<T> PointerCast<T> for TypePointer<T> {
	#[inline(always)]
	fn from(pointer: *mut T) -> VoidPointer {
		unsafe {
			*(&pointer as *const *mut _ as *const _)
		}
	}

	#[inline(always)]
	fn cast(&self) -> *mut T {
		unsafe {
			*(&self.native as *const _ as *const *mut _)
		}
	}
}

impl Add<usize> for VoidPointer {
	type Output = VoidPointer;

	#[inline(always)]
	fn add(self, other: usize) -> VoidPointer {
		VoidPointer(self.0 + (other as NativePtr))
	}
}

impl Sub<usize> for VoidPointer {
	type Output = VoidPointer;

	#[inline(always)]
	fn sub(self, other: usize) -> VoidPointer {
		VoidPointer(self.0 - (other as NativePtr))
	}
}

impl Shr<usize> for VoidPointer {
	type Output = VoidPointer;

	#[inline(always)]
	fn shr(self, rhs: usize) -> VoidPointer {
		VoidPointer(self.0 >> rhs)
	}
}

impl Shl<usize> for VoidPointer {
	type Output = VoidPointer;

	#[inline(always)]
	fn shl(self, rhs: usize) -> VoidPointer {
		VoidPointer(self.0 << rhs)
	}
}

impl Not for VoidPointer {
	type Output = VoidPointer;

	#[inline(always)]
	fn not(self) -> VoidPointer {
		VoidPointer(!self.0)
	}
}

impl<T> Deref for TypePointer<T> {
	type Target = T;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe {
			&*self.cast()
		}
	}
}

impl<T> DerefMut for TypePointer<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			&mut *self.cast()
		}
	}
}

impl<T> Index<usize> for TypePointer<T> {
	type Output = T;

	#[inline(always)]
	fn index(&self, at: usize) -> &Self::Output {
		unsafe {
			&*self.cast().wrapping_offset(at as isize)
		}
	}
}

impl<T> IndexMut<usize> for TypePointer<T> {
	#[inline(always)]
	fn index_mut(&mut self, at: usize) -> &mut Self::Output {
		unsafe {
			&mut *self.cast().wrapping_offset(at as isize)
		}
	}
}

impl Deref for VoidPointer {
	type Target = u8;

	#[inline(always)]
	fn deref(&self) -> &u8 {
		unsafe {
			&*self.cast()
		}
	}
}

impl DerefMut for VoidPointer {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut u8 {
		unsafe {
			&mut *self.cast()
		}
	}
}

impl Index<usize> for VoidPointer {
	type Output = u8;

	#[inline(always)]
	fn index(&self, at: usize) -> &u8 {
		unsafe {
			&*self.cast().wrapping_offset(at as isize)
		}
	}
}

impl IndexMut<usize> for VoidPointer {
	#[inline(always)]
	fn index_mut(&mut self, at: usize) -> &mut u8 {
		unsafe {
			&mut *self.cast().wrapping_offset(at as isize)
		}
	}
}

impl Display for VoidPointer {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{:x}", (*self).0)
	}
}
