// lib.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

//! Aldaron's Memory Interface (A.M.I.) is a Rust library for manipulating
//! memory.

#![no_std] // No Standard Library.

pub mod void_pointer;
pub mod repurpose;
pub mod size_of;
pub mod boxed;
pub mod vec;

mod heap;
