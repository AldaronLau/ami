// lib.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! Aldaron's Memory Interface is a library for manipulating memory.

#![doc(
	html_logo_url =
		"https://rawgit.com/aldarons-tech/ami/master/res/icon.png",
	html_favicon_url =
		"https://rawgit.com/aldarons-tech/ami/master/res/symbol.svg",
	html_root_url = "http://at.plopgrizzly.tech/ami/"
)]

#![no_std] // No Standard Library.

#[deprecated(since = "0.4.0", note = "Use *mut Void instead.")]
pub mod void_pointer;
#[deprecated(since = "0.4.0", note = "Use RawData::transmute instead.")]
pub mod repurpose;

mod vec;
mod void;

pub use void::*;
pub use vec::*;

/// Get the size of type `T`, in bytes.
#[inline(always)]
pub fn size_of<T>() -> usize {
	::core::mem::size_of::<T>()
}
