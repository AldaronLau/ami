// Aldaron's Memory Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/void.rs

/// Void type for C's `void *`.  Use as `*mut Void`.
#[repr(u8)]
pub enum Void {
	#[doc(hidden)]
	__DontUseMe,
	#[doc(hidden)]
	__DontUseMe2,
}
