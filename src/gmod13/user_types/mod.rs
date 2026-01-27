//! Traits for implementing user types.

use core::{
	ffi::{
		CStr,
		c_void, c_uchar,
	},
	mem::{
		MaybeUninit, needs_drop,
	},
	ptr::NonNull,
};

use super::{
	func::Rets,
	Lua,
	Type, StdType, RawType,
	StackPos,
};

mod func;
pub use func::*;

/// Base trait for [`UserType`] that will typically be implemented with [`gmod13_type!`](crate::gmod13_type!).
/// 
/// # Safety
/// `ID` must *uniquely* (on a best-effort basis) identify the implementing type,
/// the type *must not* be generic,
/// it must have an alignment `<= 8`
/// and size `<= (c_uint::MAX as usize) - size_of::<RawUd>()`.
pub unsafe trait UserTypeBase: Sized {
	/// Name of the metatable associated with this Rust type.
	const ID: &'static CStr;
	/// Error message for whenever a value of this type is expected.
	const EXPECTED_ERR: &'static CStr;
}

/// Trait for Rust types that can be sent to and returned from Lua.
pub trait UserType: UserTypeBase {
	/// Initializes the Lua type's metatable on the top of the stack,
	/// given its associated [`Type`].
	/// 
	/// You do not need to set `__gc` to handle destruction -
	/// the given metatable already has `__gc` set to run the type's destructor if needed.
	fn init_metatable(cx: SelfCtx<'_, Self>);

	/// Destroys an instance of this type,
	/// given the [`Lua`] state it's in,
	/// and its associated [`Type`].
	/// 
	/// # Safety
	/// This function must only be called *once*, just before the [`Drop`] implementation,
	/// while an instance of this type is in a Lua state.
	/// Typically, this is already done by the `__gc` metamethod provided by the crate.
	unsafe fn collect(&mut self, cx: SelfCtx<'_, Self>) {
		let _ = cx;
	}
}

/// Implements [`UserTypeBase`](crate::gmod13::user_types::UserTypeBase) for the given type.
// TODO: Is including `module_path!()` sound enough?
#[macro_export]
macro_rules! gmod13_type {
	($Type:ty) => {
		const _: () = {
			unsafe impl $crate::gmod13::user_types::UserTypeBase for $Type {
				const ID: &'static ::core::ffi::CStr = unsafe {
					::core::ffi::CStr::from_bytes_with_nul_unchecked(
						::core::concat! {
							::core::module_path!(), "::", ::core::stringify! {$Type}, '\0'
						}.as_bytes()
					)
				};
				const EXPECTED_ERR: &'static ::core::ffi::CStr = unsafe {
					::core::ffi::CStr::from_bytes_with_nul_unchecked(
						::core::concat! {
							::core::stringify! {$Type}, " expected\0"
						}.as_bytes()
					)
				};
			}

			const _ASSERT_USER_TYPE_BASE_ALIGNMENT: () = ::core::assert!(
				::core::mem::align_of::<$Type>() <= 8,
				::core::concat! {
					'`', ::core::stringify! {$Type}, "` does not meet alignment requirement for `UserTypeBase`"
				}
			);
			const _ASSERT_USER_TYPE_BASE_SIZE: () = ::core::assert!(
				::core::mem::size_of::<$Type>()
					<= (::core::ffi::c_uint::MAX as usize) - ::core::mem::size_of::<$crate::gmod13::user_types::RawUd>(),
				::core::concat! {
					'`', ::core::stringify! {$Type}, "` does not meet size requirement for `UserTypeBase`"
				}
			);
		};
	};
}

extern "C-unwind" fn user_type_gc<T: UserType>(cx: MethodFuncCtx<'_, T>) -> Rets {
	let cx = cx.lua();
	
	let mut this = cx.check_self_ptr();
	unsafe {
		T::collect(this.as_mut(), cx);
		this.drop_in_place();
	}

	Rets::ZERO
}

/// Functions for handling user types.
impl Lua {
	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	/// 
	/// `init` must initialize the given value.
	pub unsafe fn create_user_type<'a, T: UserType, F: FnOnce(&mut MaybeUninit<T>)>(
		&mut self, ty: Type, init: F,
	) -> Option<&'a mut T> {
		struct RawUdOf<T> {
			pub ud: RawUd,
			pub value: T,
		}

		let ud: *mut RawUdOf<T> = unsafe {
			self.new_userdata_raw(size_of::<RawUdOf<T>>() as _).cast()
		};
		if ud.is_null() {
			return None
		}

		let raw_ty = ty.0;
		let value_ptr = unsafe {
			let value_ptr = &raw mut (*ud).value;
			init(&mut *(value_ptr as *mut MaybeUninit<_>));
			(*ud).ud.data = value_ptr as _;
			(*ud).ud.ty = raw_ty as _;
			(*ud).ud.rust_ty = raw_ty;
			value_ptr
		};

		if self.push_metatable(ty) {
			self.set_metatable(-2);
		}

		unsafe { Some(&mut *value_ptr) }
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn push_user_type<'a, T: UserType>(&mut self, ty: Type, value: T) -> Option<&'a mut T> {
		unsafe { self.create_user_type(ty, move |init| { init.write(value); }) }
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn test_ud_ptr<T: UserType>(&self, ty: Type, stack_pos: StackPos) -> Option<NonNull<T>> {
		if !self.is_type(stack_pos, ty) {
			return None
		}

		let ud = unsafe { self.get_userdata(stack_pos).cast::<RawUd>().as_ref()? };
		if ud.ty != ty.0 as _ && ud.rust_ty != ty.0 {
			return None
		}

		NonNull::new(ud.data.cast::<T>())
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn check_ud_ptr<T: UserType>(&self, ty: Type, arg: StackPos) -> NonNull<T> {
		match unsafe { self.test_ud_ptr(ty, arg) } {
			Some(t) => t,
			None => self.arg_error(arg, T::EXPECTED_ERR),
		}
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn test_ud<T: UserType>(&self, ty: Type, stack_pos: StackPos) -> Option<&T> {
		Some(unsafe { self.test_ud_ptr(ty, stack_pos)?.as_ref() })
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn test_ud_mut<T: UserType>(&mut self, ty: Type, stack_pos: StackPos) -> Option<&mut T> {
		Some(unsafe { self.test_ud_ptr(ty, stack_pos)?.as_mut() })
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn check_ud<T: UserType>(&self, ty: Type, arg: StackPos) -> &T {
		unsafe { self.check_ud_ptr(ty, arg).as_ref() }
	}

	/// # Safety
	/// `ty` must be the correct type identifier for `T`.
	pub unsafe fn check_ud_mut<T: UserType>(&mut self, ty: Type, arg: StackPos) -> &mut T {
		unsafe { self.check_ud_ptr(ty, arg).as_mut() }
	}

	pub fn register<T: UserType>(&mut self) -> Type {
		let ty = self.create_metatable(T::ID);
		let ty_raw = ty.0 as _;

		self.push_registry();
		push_registry_key::<T>(self);
		self.push_bits(ty_raw);
		self.raw_set(-3); // registry[key] = ty
		self.pop(1);

		let mut cx = unsafe { SelfCtx::new(self, ty) };
		if needs_drop::<T>() {
			cx.push_method(user_type_gc::<T>);
			cx.set_field(-2, c"__gc");
		}
		T::init_metatable(cx);

		ty
	}

	/// Returns the [`Type`] of the Lua user type associated with `T`.
	/// 
	/// # Errors
	/// This function will raise an [error](crate::errors)
	/// if `T` has not been [`register`](Self::register)ed.
	pub fn user_type_of<T: UserType>(&self) -> Type {
		self.push_registry();
		push_registry_key::<T>(self);
		self.raw_get(-2); // registry[key]

		if !self.is_type(-1, StdType::Number) {
			self.throw_error(c"type does not have an associated type ID in this Lua state")
		}

		let raw_ty = self.get_bits(-1);
		self.pop(1);
		Type(raw_ty as _)
	}
}

fn push_registry_key<T: UserType>(lua: &Lua) {
	unsafe { lua.push_light_userdata(T::ID.as_ptr() as *mut ()) }
}

/// Raw header for userdata allocated in a Lua state.
pub struct RawUd {
	pub data: *mut c_void,
	pub ty: c_uchar,
	pub rust_ty: RawType,
}
