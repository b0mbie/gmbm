use core::{
	marker::PhantomData,
	mem::transmute,
	ops::{
		Deref, DerefMut,
	},
	ptr::NonNull,
};

use super::{
	super::{
		func::{
			Func, Ctx, Rets,
		},
		Lua, Type,
	},
	UserType,
};

/// Context for function calls with a `self` of type `T`.
pub struct SelfCtx<'a, T> {
	lua: &'a mut Lua,
	ty: Type,
	_t: PhantomData<fn(*const T)>,
}

impl<'a, T: UserType> SelfCtx<'a, T> {
	pub(super) const unsafe fn new(lua: &'a mut Lua, ty: Type) -> Self {
		Self {
			lua, ty,
			_t: PhantomData,
		}
	}

	/// Returns the [`Type`] for `self`.
	pub fn self_ty(&self) -> Type {
		self.ty
	}

	/// Returns a pointer to `self` as a `T`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors)
	/// if the `self` argument is not `T`.
	pub fn check_self_ptr(&self) -> NonNull<T> {
		unsafe { self.check_ud_ptr(self.ty, 1) }
	}

	/// Returns an immutable reference to `self` as a `T`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors)
	/// if the `self` argument is not `T`.
	pub fn check_self(&self) -> &T {
		unsafe { self.check_ud(self.ty, 1) }
	}

	/// Returns a mutable reference to `self` as a `T`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors)
	/// if the `self` argument is not `T`.
	pub fn check_self_mut(&mut self) -> &mut T {
		let ty = self.ty;
		unsafe { self.check_ud_mut(ty, 1) }
	}

	/// Pushes the given method function onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_method(&mut self, f: MethodFunc<T>) {
		self.lua.push_bits(self.ty.0 as _);
		self.lua.push_closure(to_func(f), 1)
	}
}

impl<T> Deref for SelfCtx<'_, T> {
	type Target = Lua;
	fn deref(&self) -> &Self::Target {
		self.lua
	}
}
impl<T> DerefMut for SelfCtx<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.lua
	}
}

/// Returns a [`MethodFunc`] that can be called by Lua.
#[macro_export]
macro_rules! gmod13_method {
	($T:ty => $lua:pat => $body:block) => {{
		extern "C-unwind" fn __gmod13_method_inline(
				cx: $crate::gmod13::user_types::MethodFuncCtx<'_, $T>,
			) -> $crate::gmod13::func::Rets {
			let $lua = cx.lua();
			<$crate::gmod13::func::Rets as ::core::convert::From<_>>::from($body)
		}
		__gmod13_method_inline
	}};

	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `<Type> => <pattern> => <body>`"
		}
	};
}

/// [`Func`] that is intended to be called on a `self` of type `T`.
pub type MethodFunc<T> = extern "C-unwind" fn(MethodFuncCtx<'_, T>) -> Rets;

const fn to_func<T: UserType>(f: MethodFunc<T>) -> Func {
	unsafe { transmute(f) }
}

/// Context passed to a [`MethodFunc`].
/// 
/// # Layout
/// This type has the same layout and ABI as [`Ctx<'a>`].
#[repr(transparent)]
pub struct MethodFuncCtx<'a, T> {
	cx: Ctx<'a>,
	_t: PhantomData<fn() -> T>
}

impl<'a, T: UserType> MethodFuncCtx<'a, T> {
	pub fn lua(self) -> SelfCtx<'a, T> {
		let lua = self.cx.lua();
		let ty = {
			lua.push_upvalue(0);
			let ty = Type(lua.get_bits(-1) as _);
			lua.pop(1);
			ty
		};
		unsafe { SelfCtx::new(lua, ty) }
	}
}
