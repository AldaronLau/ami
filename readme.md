# [Aldaron's Memory Interface](https://crates.io/crates/ami)
Aldaron's Memory Interface provides useful data structures that are not in the
standard library.

## Features
* Provide geometrical data structs, and do math with them
* Automatic-size-adjusting octree

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* All data structures do what they are supposed to.
* 32-bit-indexed Vec (`VecD`).

## Change Log
### 0.11
* Update to euler 0.4
* Removed Parent and Child structs (use `Rc<RefCell>` instead)
* Removed casting macros

### 0.10
* Switch to using Euler for `Vec*` types.

### 0.9
* Fixed Octree bugs.
* Made `Mat4` use f32s instead of f64s

### 0.8
* Octree now uses `BBox` instead of Vec3 for positioning.
* Renamed `BBox` to `BCube`.
* Added `BBox`.
* Renamed `Pos` to `Collider`.
* A few other changes.

## Developed by [Plop Grizzly](http://plopgrizzly.com)
