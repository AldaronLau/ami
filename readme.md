# [Aldaron's Memory Interface](https://crates.io/crates/ami)
Aldaron's Memory Interface provides data structures and casting macros.

## Features
**ami**'s current features:
* Casting pointers with the `cast!()` and `cast_mut!()` macros
* Provide geometrical data structs, an do math with them
* Automatic-size-adjusting octree

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* Remove unneeded features.
* Use num-trait for `Vec*` types

## Change Log
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
