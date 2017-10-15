# Aldaron's Memory Interface 0.5.0
Aldaron's Memory Interface is a library developed by Plop Grizzly for
manipulating memory.

[Cargo](https://crates.io/crates/ami) /
[Documentation](https://docs.rs/ami)

## Example
Using **ami**'s `Vec`:

```rust
extern crate ami;

fn main() {
	let mut vec : ami::Vec<i32> = ami::Vec::new();

	println!("Empty Vec: {}", vec);
	vec.push(12);
	println!("Pushed 12: {}", vec);
	vec.push(102);
	println!("Pushed 102: {}", vec);
	println!("vec[1] = {}", vec[1]);
	println!("Pop: {}", vec.pop().unwrap());
	println!("Vector: {}", vec);
	println!("vec[0] = {}", vec[0]);
}
```

## Features
**ami**'s current features:
* Provide C's `void *` type
* Provide C's `NULL` as `null!`.
* Safe dynamic memory allocation without rust's std lib.
* Safe `Vec` operations without rust's std lib.
* Casting `Void` pointers like you would in C.
* Transmute without rust's std lib.

**ami**'s planned features:
* Load .so's into memory without libc/libdl
* Load dll's into memory without libc/libdl

## Support
**ami**'s current support:
* Platforms with libc

**ami**'s planned support:
* Arduino and Raspberry Pi (no os), providing a dynamic allocation
implementation.
