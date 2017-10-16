// Aldaron's Memory Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Memory Interface is a library developed by Plop Grizzly for
//! manipulating memory.

#![no_std] // No Standard Library.
#![warn(missing_docs)]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/plopgrizzly\
		/ami/master/res/icon.png",
	html_favicon_url = "https://raw.githubusercontent.com/plopgrizzly\
		/ami/master/res/symbol.svg",
	html_root_url = "http://plopgrizzly.com/ami/"
)]

#[macro_use]
mod void;
mod vec;
mod heap_mem;
mod mem_addr;

pub use void::Void;
pub use vec::Vec;
pub use heap_mem::HeapMem;
pub use mem_addr::MemAddr;

#[doc(hidden)]
pub mod hidden_core {
	pub use ::core::ptr::{ null, null_mut };
}

/// Get the size of type `T`, in bytes.
#[inline(always)]
pub fn size_of<T>() -> usize {
	::core::mem::size_of::<T>()
}

/// Copy bits ( raw data ) from T to U.  Make sure types are same size.
/// This function is unsafe because it does no checking.
#[inline(always)]
pub unsafe fn transmute<T,U>(from: T) -> U {
	::core::ptr::read(&from as *const _ as *const U)
}

/// Cast a constant pointer to another type.
#[macro_export] macro_rules! cast {
	($a:expr) => {
		$a as *const _ as *const _
	}
}

/// Cast a mutable pointer to another type.
#[macro_export] macro_rules! cast_mut {
	($a:expr) => {
		$a as *mut _ as *mut _
	}
}

/// Obtain a null pointer.
#[macro_export] macro_rules! null {
	() => {
		$crate::hidden_core::null()
	}
}

/// Obtain a mutable null pointer.
#[macro_export] macro_rules! null_mut {
	() => {
		$crate::hidden_core::null_mut()
	}
}
