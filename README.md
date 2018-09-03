[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)

# [Aldaron's Memory Interface](https://crates.io/crates/ami)
Aldaron's Memory Interface provides useful data structures that are not in the
standard library.

## Features
* 3D Vector Math
* 4D Matrix Math
* Other geometrical math, and related structures
* Automatic-size-adjusting octree

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* All data structures do what they are supposed to.
* 32-bit-indexed Vec (`VecD`).

## Change Log
### 0.13
* No longer depends on `euler`, but a lower level crate `cgmath`.
* Rename `vec3!` to `vector!`, and `Vec3` to `Vector`.
* Rename `mat4!` to `matrix!`, and `Mat4` to `Matrix`.
* Now uses proper quaternion math

### 0.12
* Replace macros module with prelude module.

### 0.11
* Update to euler 0.4
* Removed Parent and Child structs (use `Rc<RefCell>` instead)
* Removed casting macros

### 0.10
* Switch to using Euler for `Vec*` types.

### 0.9
* Fixed Octree bugs.
* Made `Mat4` use f32s instead of f64s
