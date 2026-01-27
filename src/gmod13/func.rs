//! Safer APIs for callable native functions.

use core::{
	ffi::c_int,
	marker::PhantomData,
	mem::transmute,
};

use super::{
	CFunc,
	LuaState, Lua,
};

/// Converts a [`Func`] to a [`CFunc`].
pub const fn to_c_func(f: Func) -> CFunc {
	// SAFETY: `Func` is ABI-compatible with `CFunc`.
	unsafe { transmute(f) }
}

/// Rust function that can be called by Lua.
/// 
/// This type is compatible with [`CFunc`];
/// however, this may not be a safe assumption to make for the reverse.
/// Use [`to_c_func`] to convert from this type if needed.
pub type Func = extern "C-unwind" fn(cx: Ctx<'_>) -> Rets;

/// Returns a [`Func`] that can be called by Lua,
/// given an inline function definition similar to a Rust closure.
/// 
/// # Examples
/// ```
/// # use gmbm::{Func, gmod13_fn};
/// let _: Func = gmod13_fn!(mut lua => {
///     lua.push_string("Hey every    !");
///     1
/// });
/// ```
#[macro_export]
macro_rules! gmod13_fn {
	($lua:pat => $body:block) => {{
		extern "C-unwind" fn __gmod13_fn_inline(cx: $crate::gmod13::func::Ctx) -> $crate::gmod13::func::Rets {
			let $lua = cx.lua();
			<$crate::gmod13::func::Rets as ::core::convert::From<_>>::from($body)
		}
		__gmod13_fn_inline
	}};

	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `<pattern> => <body>`"
		}
	};
}

/// Context passed to a [`Func`].
/// 
/// # Layout
/// This type has the same layout and ABI as [`*mut LuaState`](LuaState).
#[repr(transparent)]
pub struct Ctx<'a> {
	ptr: *mut LuaState,
	_life: PhantomData<fn() -> &'a Lua>,
}

impl<'a> Ctx<'a> {
	/// Converts this context into [`Lua`].
	pub const fn lua(self) -> &'a mut Lua {
		unsafe { Lua::from_mut_ptr(self.ptr) }
	}
}

/// Type for the number of values returned from a [`Func`].
/// 
/// # Layout
/// This type has the same layout and ABI as [`c_int`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Rets {
	// INVARIANT: This field is non-negative.
	count: c_int,
}

impl Rets {
	/// Constant that indicates no return values.
	pub const ZERO: Self = Self::new(0);

	/// Creates a new [`Rets`] from the specified number of returned values.
	pub const fn new(n: usize) -> Self {
		let count = if n <= c_int::MAX as usize {
			n as c_int
		} else {
			0
		};
		unsafe { Self::new_unchecked(count) }
	}

	/// Creates a new [`Rets`] without checking whether
	/// the number of returned values is non-negative.
	/// 
	/// # Safety
	/// `count` must be non-negative.
	pub const unsafe fn new_unchecked(count: c_int) -> Self {
		Self { count, }
	}
}

impl From<()> for Rets {
	fn from(_: ()) -> Self {
		Self::ZERO
	}
}

impl From<usize> for Rets {
	fn from(value: usize) -> Self {
		Self::new(value)
	}
}
