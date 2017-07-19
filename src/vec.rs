// vec.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use core::ptr;
use core::marker::PhantomData;

use NULL;
use Void;
use UnsafeData;
use size_of;

/// A growable array on the heap.
pub struct Vec<T> {
	ptr: UnsafeData,
	cap: usize,
	len: usize,
	marker: PhantomData<T>,
}

impl<T> Vec<T> {
	/// Create an empty `Vec<T>`.
	#[inline(always)]
	pub fn new() -> Vec<T> {
		let ptr = NULL;
		let cap = 0;
		let len = 0;

		Vec { ptr, cap, len, marker: PhantomData }
	}

	/// Append an element at the end of the `Vec<T>`.
	#[inline(always)]
	pub fn push(&mut self, elem: T) -> () {
		// If it needs to grow, re-allocate.
		if self.grow() {
			self.resize();
		}

		// Initialize the uninitialized.
		unsafe {
			self.ptr.as_slice(self.len + 1)[self.len] = elem;
		}

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

		let nlen = self.len - 1;

		// This is safe because we're moving the value out of the vector
		// The copied value is out of bounds, so it's a move.
		let element = unsafe {
			ptr::read(self.as_ptr().wrapping_offset(nlen as isize))
		};

		// Length has decreased by one.
		self.len = nlen;

		Some(element)
	}

	/// Get a raw pointer to the `Vec<T>`'s Buffer.
	#[inline(always)]
	pub fn as_ptr(&self) -> *const T {
		self.ptr.as_ptr()
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

	// Resize ptr from capacity.
	#[inline(always)]
	fn resize(&mut self) {
		let mut ptr = self.ptr.as_mut_ptr();
		let bytes = self.cap * size_of::<T>();

		self.ptr = unsafe {
			Void::resize(&mut ptr, bytes);
			UnsafeData::new(ptr)
		};
	}
}

impl<T> Drop for Vec<T> {
	#[inline(always)]
	fn drop(&mut self) {
		if self.cap != 0 {
			unsafe {
				Void::drop(self.ptr.as_mut_ptr());
			}
		}
	}
}

impl<T> ::core::fmt::Display for Vec<T> where T: ::core::fmt::Display {
	#[inline(always)]
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		let mut comma = false;
		write!(f, "[")?;

		for i in self.iter() {
			if comma {
				write!(f, ", ")?;
			} else {
				comma = true;
			}

			write!(f, "{}", i)?;
		}
		write!(f, "]")
	}
}

impl<T> ::core::ops::Deref for Vec<T> {
	type Target = [T];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe {
			let ptr = self.ptr.as_ptr();

			::core::slice::from_raw_parts(ptr, self.len)
		}
	}
}

impl<T> ::core::ops::DerefMut for Vec<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			let ptr = self.ptr.as_mut_ptr();

			::core::slice::from_raw_parts_mut(ptr, self.len)
		}
	}
}

impl<T> ::core::ops::Index<usize> for Vec<T> {
	type Output = T;

	#[inline(always)]
	fn index(&self, index: usize) -> &Self::Output {
		let elem_ptr = self.as_ptr().wrapping_offset(index as isize);

		if index >= self.len {
			panic!("Couldn't index vector: Index {}, but size {}",
				index, self.len)
		} else {
			unsafe { & *elem_ptr }
		}
	}
}

impl<T> ::core::ops::IndexMut<usize> for Vec<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		let elem_ptr = self.as_mut_ptr().wrapping_offset(index as isize);

		if index >= self.len {
			panic!("Couldn't index vector: Index {}, but size {}",
				index, self.len)
		} else {
			unsafe { &mut *elem_ptr }
		}
	}
}
