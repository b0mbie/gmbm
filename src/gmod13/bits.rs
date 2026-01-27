use super::*;

mod private {
	pub trait AsBits { type Bits; }
	impl AsBits for f32 { type Bits = u32; }
	impl AsBits for f64 { type Bits = u64; }
}
use private::*;

/// Integer type that can be losslessly converted to a [`Number`] and back.
/// 
/// Values of this type in the Lua state have the same *bitwise* value,
/// but often not the same *numerical* value.
/// This type can be used for losslessly storing large integers in Lua.
pub type Bits = <Number as AsBits>::Bits;

/// Functions for losslessly storing large integers in Lua.
impl Lua {
	/// Returns the [`Bits`] encoded as a Lua number at `stack_pos`,
	/// or `0` if the value isn't a Lua number.
	pub fn get_bits(&self, stack_pos: StackPos) -> Bits {
		self.get_number(stack_pos).to_bits()
	}

	/// If the value at `stack_pos` is a [`Number`], decodes it as [`Bits`].
	/// Otherwise, throws an error.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_bits(&self, stack_pos: StackPos) -> Bits {
		self.check_number(stack_pos).to_bits()
	}

	/// Pushes the given integer onto the stack as a Lua number.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_bits(&self, i: Bits) {
		self.push_number(Number::from_bits(i))
	}
}
