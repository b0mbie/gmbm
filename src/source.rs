//! Source Engine structures.

use core::ffi::c_float;

/// Vector component type.
/// 
/// From `basetypes.h` in `tier0`.
#[allow(non_camel_case_types)]
type vec_t = c_float;

/// Source Engine 3D vector type.
/// 
/// Cross-referenced with `vector.h` in `mathlib`.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
	pub x: vec_t,
	pub y: vec_t,
	pub z: vec_t,
}

impl Vector {
	/// Create a new 3D vector from its components.
	pub const fn new(x: vec_t, y: vec_t, z: vec_t) -> Self {
		Self {
			x, y, z,
		}
	}
}

/// Vector that represents three-dimensional extrinsic Tait-Bryan rotations following the right-hand rule,
/// offset from the cardinal Z axis.
pub type QAngle = Vector;
