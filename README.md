# Aldaron's Memory Interface ![](res/icon.png)

Aldaron's Memory Interface is a library developed by Plop Grizzly for manipulating memory.

## Support
Aldaron's Memory Interface supports:
* Platforms with libc

Aldaron's Memory Interface will support:
* Single program devices without libc by using own implementation of dynamic allocation.

## Features
Aldaron's Memory Interface can:
* Give you C's `void *` type
* Give you C's `NULL` as `null!`.
* Allocate dynamic memory safely.
* Do safe `Vec` operations that don't require rust's std lib.
* Cast `Void` pointers like you would in C.
* Transmute without rust's std lib.

Aldaron's Memory Interface will be able to:
* Load .so's into memory without libc/libdl
* Load dll's into memory without libc/libdl

## Links
* [Website](http://plopgrizzly.com/ami)
* [Cargo](https://crates.io/crates/ami)
* [Documentation](https://docs.rs/ami)
* [Tutorial](https://plopgrizzly.gitbooks.io/ami)
