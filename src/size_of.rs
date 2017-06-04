// repurpose.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! This module is for getting the size of a variable.

/// Get the size of type `T`, in bytes.
#[inline(always)]
pub fn size_of<T>() -> usize {
	::core::mem::size_of::<T>()
}
