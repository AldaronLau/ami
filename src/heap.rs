// heap.rs
// Aldaron's Memory Interface ( ami )
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use void_pointer::*;

// Allocate memory.
#[inline(always)]
pub fn alloc(ptr: VoidPointer, bytes: usize) -> VoidPointer {
	extern "C" {
		fn realloc(ptr: VoidPointer, size: usize) -> VoidPointer;
	}

	let memory = unsafe {
		realloc(ptr, bytes)
	};

	if memory == NULL {
		panic!("Error: out of memory");
	}

	memory
}
