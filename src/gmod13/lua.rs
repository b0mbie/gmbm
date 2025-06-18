use core::{
	cmp::Ordering,
	error::Error,
	ffi::{
		CStr,
		c_int, c_uint, c_double, c_void,
	},
	fmt,
	marker::PhantomData,
	mem::MaybeUninit,
	slice::from_raw_parts_mut as slice_from_raw_parts_mut,
};
use cppdvt::{
	VtObject, virtual_call,
};

use crate::source::{
	Vector, QAngle,
};

use super::cppdef::*;

/// Interface for the Lua environment of
/// the same Garry's Mod version that uses `gmod13_open` and `gmod13_close` functions for binary modules.
#[derive(Debug)]
#[repr(transparent)]
pub struct Lua<'a> {
	luabase: VtObject<LuaBaseVt>,
	_life: PhantomData<&'a mut ()>,
}

impl Lua<'_> {
	/// See [`LuaState`].
	/// 
	/// # Safety
    /// `api_ptr` must be a valid `lua_State` pointer from the Garry's Mod version this structure targets.
	pub const unsafe fn new(api_ptr: VtObject<LuaBaseVt>) -> Self {
		Self {
			luabase: api_ptr,
			_life: PhantomData,
		}
	}

	/// Return the amount of values on the stack.
	pub fn top(&self) -> c_uint {
		(unsafe { virtual_call!(self.luabase, top) }) as _
	}

	/// Push a copy of the value at `stack_pos` to the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_value(&self, stack_pos: c_int) {
		unsafe { virtual_call!(self.luabase, push, stack_pos) }
	}

	/// Pop `n` values from the stack.
	pub fn pop(&self, amt: c_uint) {
		// SAFETY: To-be-closed slots aren't a thing in Lua 5.1 and LuaJIT.
		unsafe { virtual_call!(self.luabase, pop, amt as _) }
	}

	/// Push the value `t[key]`,
	/// where `t` is the value at `stack_pos`,
	/// and `key` is the value popped from the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn get_table(&mut self, stack_pos: c_int) {
		unsafe { virtual_call!(self.luabase, get_table, stack_pos) }
	}

	/// Push the value `t[key]`,
	/// where `t` is the value at `stack_pos`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn get_field(&mut self, stack_pos: c_int, key: &CStr) {
		unsafe { virtual_call!(self.luabase, get_field, stack_pos, key.as_ptr()) }
	}

	/// Set `t[key]` to the value popped from the stack,
	/// where `t` is the value at `stack_pos`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_field(&mut self, stack_pos: c_int, key: &CStr) {
		unsafe { virtual_call!(self.luabase, set_field, stack_pos, key.as_ptr()) }
	}

	/// Create a new table and push it onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn create_table(&mut self) {
		unsafe { virtual_call!(self.luabase, create_table) }
	}

	/// Set `t[key]` to the value popped from the stack,
	/// where `t` is the value at `stack_pos`,
	/// and `key` is the value popped from just below the top of the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_table(&mut self, stack_pos: c_int) {
		unsafe { virtual_call!(self.luabase, set_table, stack_pos) }
	}

	/// Set the metatable for the value at `stack_pos` to the value popped from the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_metatable(&mut self, stack_pos: c_int) {
		unsafe { virtual_call!(self.luabase, set_meta_table, stack_pos) }
	}

	/// Push the metatable of the value at `stack_pos` on the stack, returning `true`,
	/// or push nothing and return `false` on failure.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn get_metatable(&mut self, stack_pos: c_int) -> bool {
		unsafe { virtual_call!(self.luabase, get_meta_table, stack_pos) }
	}

	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn call(&mut self, n_args: c_uint, n_results: c_uint) {
		unsafe { virtual_call!(self.luabase, call, n_args as _, n_results as _) }
	}

	/// Call an object as a function on the stack, returning `true` 
	/// 
	/// The algorithm to call an object is as follows:
	/// 1. Pop `n_args` values from the stack for the call arguments.
	/// 2. Pop the callee from the stack.
	pub fn pcall(&mut self, n_args: c_uint, n_results: c_int, error_func: c_int) -> Result<(), CallError> {
		let result = unsafe { virtual_call!(self.luabase, pcall, n_args as _, n_results, error_func) };
		if result == 0 {
			Ok(())
		} else {
			Err(CallError)
		}
	}

	/// Return `true` if the values at `a` and `b` are equal.
	/// 
	/// See also [`Lua::raw_equal`].
	pub fn equal(&self, a: c_int, b: c_int) -> bool {
		(unsafe { virtual_call!(self.luabase, equal, a, b) }) != 0
	}

	/// Return `true` if the values at `a` and `b` are equal,
	/// without invoking metamethods.
	/// 
	/// See also [`Lua::equal`].
	pub fn raw_equal(&self, a: c_int, b: c_int) -> bool {
		(unsafe { virtual_call!(self.luabase, raw_equal, a, b) }) != 0
	}
	
	/// Move the value at the top of the stack into `stack_pos`,
	/// shifting upwards any elements above `stack_pos`.
	pub fn insert(&mut self, stack_pos: c_int) {
		unsafe { virtual_call!(self.luabase, insert, stack_pos) }
	}

	/// Remove the value at `stack_pos`,
	/// shifting values above `stack_pos` downwards.
	pub fn remove(&self, stack_pos: c_int) {
		unsafe { virtual_call!(self.luabase, remove, stack_pos) }
	}

	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	// TODO: Describe functionality.
	pub fn next(&mut self, stack_pos: c_int) -> c_int {
		unsafe { virtual_call!(self.luabase, next, stack_pos) }
	}

	/// Throw an error and cease execution of the function.
	pub fn throw_error(&self, message: &'static CStr) -> ! {
		unsafe { virtual_call!(self.luabase, throw_error, message.as_ptr()) }
	}

	/// Throw an error if the value at `stack_pos` is not of the given [`Type`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_type(&self, stack_pos: c_int, ty: impl Into<Type>) {
		unsafe { virtual_call!(self.luabase, check_type, stack_pos, ty.into().0) }
	}

	/// Throw an error related to argument `arg_num` and cease execution of the function.
	pub fn arg_error(&self, arg_num: c_int, message: &'static CStr) -> ! {
		unsafe { virtual_call!(self.luabase, arg_error, arg_num, message.as_ptr()) }
	}

	// TODO: arg_error
	// TODO: raw_get
	// TODO: raw_set
	// TODO: get_string
	// TODO: get_number
	// TODO: get_bool
	// TODO: get_c_function

	/// Return the non-null pointer to the userdata at `stack_pos`,
	/// or a null pointer if the value isn't userdata.
	pub fn get_userdata(&self, stack_pos: c_int) -> *mut c_void {
		unsafe { virtual_call!(self.luabase, get_userdata, stack_pos) }
	}

	/// Push a `nil` onto the stack as a Lua string.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_nil(&mut self) {
		unsafe { virtual_call!(self.luabase, push_nil) }
	}

	/// Push the given non-empty slice of bytes onto the stack as a Lua string.
	/// 
	/// This is a function specialized to a current limitation of the API.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	/// 
	/// # Safety
	/// `non_empty_bytes` must not be empty.
	pub unsafe fn push_non_empty_bytes(&mut self, non_empty_bytes: &[u8]) {
		unsafe {
			virtual_call!(
				self.luabase, push_string,
				non_empty_bytes.as_ptr() as *const _, non_empty_bytes.len() as _
			)
		}
	}

	/// Push the given byte string onto the stack as a Lua string.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_string(&mut self, bytes: impl AsRef<[u8]>) {
		let bytes_ref = bytes.as_ref();
		// If length is `0`, `strlen` is used. Which is very, very bad!
		if !bytes_ref.is_empty() {
			unsafe { self.push_non_empty_bytes(bytes_ref) }
		} else {
			unsafe { virtual_call!(self.luabase, push_string, c"".as_ptr(), 0) }
		}
	}

	/// Push the given C string onto the stack as a Lua string.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_c_string(&mut self, string: impl AsRef<CStr>) {
		let c_string = string.as_ref();
		// TODO: Is it OK if `count_bytes` replaces the internal `strlen` calculation?
		unsafe { virtual_call!(self.luabase, push_string, c_string.as_ptr() as *const _, c_string.count_bytes() as _) }
	}

	/// Push the given number onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_number(&mut self, n: c_double) {
		unsafe { virtual_call!(self.luabase, push_number, n) }
	}

	/// Push the given boolean onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_bool(&mut self, b: bool) {
		unsafe { virtual_call!(self.luabase, push_bool, b) }
	}
	
	/// Push the given C function onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_c_function(&self, func: CFunc) {
		unsafe { virtual_call!(self.luabase, push_c_function, func) }
	}

	/// Push the given C function onto the stack with `n_upvalues`,
	/// which must be on the top of the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_c_closure(&self, func: CFunc, n_upvalues: u8) {
		unsafe { virtual_call!(self.luabase, push_c_closure, func, n_upvalues as _) }
	}

	/// Push the light userdata `ptr` onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	/// 
	/// # Safety
	/// `ptr` is a plain pointer value that can be accessed at any later point in time,
	/// and so the exact guarantees for `ptr` vary depending on the use-case.
	/// Consider using full userdata instead if you can.
	pub unsafe fn push_light_userdata(&mut self, ptr: *mut c_void) {
		virtual_call!(self.luabase, push_userdata, ptr)
	}
	
	/// Pop a value from the stack,
	/// and return a [`LuaRef`] that can be used to access it later.
	/// 
	/// See also [`Lua::push_ref`] and [`Lua::free_ref`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn create_ref(&mut self) -> LuaRef {
		let index = unsafe { virtual_call!(self.luabase, reference_create) };
		LuaRef(index)
	}

	/// Free the reference `lua_ref` if it is valid.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn free_ref(&mut self, lua_ref: LuaRef) {
		unsafe { virtual_call!(self.luabase, reference_free, lua_ref.0) }
	}

	/// Push the value pointed to by `lua_ref` onto the stack,
	/// or `nil` if the reference is invalid.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_ref(&self, lua_ref: LuaRef) {
		unsafe { virtual_call!(self.luabase, reference_push, lua_ref.0) }
	}

	/// Push a [`Special`] value onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_special(&mut self, what: Special) {
		unsafe { virtual_call!(self.luabase, push_special, what as _) }
	}

	/// Return `true` if the value at `stack_pos` is of the given [`Type`].
	pub fn is_type(&self, stack_pos: c_int, ty: impl Into<Type>) -> bool {
		unsafe { virtual_call!(self.luabase, is_type, stack_pos, ty.into().0) }
	}

	/// Return the [`Type`] of the value at `stack_pos`.
	pub fn get_type(&self, stack_pos: c_int) -> Type {
		unsafe { Type(virtual_call!(self.luabase, get_type, stack_pos)) }
	}
	
	/// Return the name of the given [`StdType`], as a C string.
	pub fn get_type_name(&self, ty: StdType) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.luabase, get_type_name, ty as _)) }
	}

	/// If the value at `stack_pos` is a string, return it.
	/// Otherwise, throw an error.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_string(&self, stack_pos: c_int) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.luabase, check_string, stack_pos)) }
	}

	/// If the value at `stack_pos` is a number, return it.
	/// Otherwise, throw an error.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_number(&self, stack_pos: c_int) -> c_double {
		unsafe { virtual_call!(self.luabase, check_number, stack_pos) }
	}

	/// Return the length of the object at `stack_pos`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn obj_len(&mut self, stack_pos: c_int) -> c_int {
		unsafe { virtual_call!(self.luabase, obj_len, stack_pos) }
	}

	/// If the value at `stack_pos` is a [`QAngle`], return a reference to it.
	/// Otherwise, return a reference to the angle `0, 0, 0`.
	pub fn get_angle(&self, stack_pos: c_int) -> &QAngle {
		unsafe { virtual_call!(self.luabase, get_angle, stack_pos).as_ref() }
	}

	/// If the value at `stack_pos` is a [`Vector`], return a reference to it.
	/// Otherwise, return a reference to the vector `0, 0, 0`.
	pub fn get_vector(&self, stack_pos: c_int) -> &Vector {
		unsafe { virtual_call!(self.luabase, get_vector, stack_pos).as_ref() }
	}

	/// Push `angle` onto the stack as a Lua object.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_angle(&mut self, angle: &QAngle) {
		unsafe { virtual_call!(self.luabase, push_angle, angle) }
	}

	/// Push `vector` onto the stack as a Lua object.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_vector(&mut self, vector: &Vector) {
		unsafe { virtual_call!(self.luabase, push_vector, vector) }
	}

	/// Push the metatable associated with the given `name`,
	/// creating it if it doesn't exist,
	/// and return the [`Type`] to use for it.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn create_metatable(&mut self, name: &CStr) -> Type {
		unsafe { Type(virtual_call!(self.luabase, create_meta_table, name.as_ptr())) }
	}
	
	/// Push the metatable associated with the given [`Type`],
	/// returning `true` if it exists.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	// TODO: What happens when it doesn't exist?
	pub fn push_metatable(&mut self, ty: impl Into<Type>) -> bool {
		unsafe { virtual_call!(self.luabase, push_meta_table, ty.into().0) }
	}

	// TODO: Something with UserTypes?
	// TODO: push_user_type
	// TODO: set_user_type

	/// Allocate a new Lua userdata of the specified `size`,
	/// returning the opaque pointer to it,
	/// which may be null if allocation failed.
	/// 
	/// # Safety
	/// Lua ensures that the pointer is valid as long as the corresponding userdata is alive.
	/// Moreover, if the userdata is marked for finalization,
	/// it is valid at least until the call to its finalizer.
	/// Do not use the returned pointer outside of these two specific circumstances!
	pub unsafe fn new_userdata_raw(&mut self, size: c_uint) -> *mut c_void {
		virtual_call!(self.luabase, new_userdata, size)
	}

	/// Allocate a new Lua userdata of the specified `size`,
	/// returning a mutable reference to its uninitialized contents if allocation succeeded.
	pub fn new_userdata(&mut self, size: c_uint) -> Option<&mut [MaybeUninit<u8>]> {
		// SAFETY: We will only use this pointer through a reference tied to `self`.
		let ptr = unsafe { self.new_userdata_raw(size) };
		if !ptr.is_null() {
			// If `ptr` is non-null, we assume that it is valid for reading and writing.
			unsafe { Some(slice_from_raw_parts_mut(ptr as *mut MaybeUninit<u8>, size as _)) }
		} else {
			None
		}
	}
}

impl Lua<'_> {
	/// Drain the stack so that it has *at most* a specific number of elements.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::pop`] and [`Lua::top`] for convenience.
	pub fn drain_to(&self, top: c_uint) {
		self.pop(self.top().saturating_sub(top))
	}

	/// Set the number of elements in the stack,
	/// filling any excess slots with `nil`.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::pop`], [`Lua::top`] and [`Lua::push_nil`] for convenience.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_top(&mut self, top: c_uint) {
		let current_top = self.top();
		match top.cmp(&current_top) {
			Ordering::Less => self.pop(current_top - top),
			Ordering::Greater => { 
				for _ in 0..top - current_top {
					self.push_nil();
				}
			}
			Ordering::Equal => {}
		}
	}

	/// Push the globals table onto the stack.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::push_special`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_globals(&mut self) {
		self.push_special(Special::Glob)
	}

	/// Push the environment table onto the stack.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::push_special`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_environment(&mut self) {
		self.push_special(Special::Env)
	}

	/// Push the registry table onto the stack.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::push_special`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_registry(&mut self) {
		self.push_special(Special::Registry)
	}

	/// Push the `n`-th upvalue onto the stack, starting from `0`,
	/// or `nil` if the upvalue index is invalid.
	/// 
	/// This method is not part of the public C++ API.
	/// It is based on LuaJIT, and may break at any time
	/// (though, it is unlikely).
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_upvalue(&mut self, n: u8) {
		self.push_value(upvalue_index(n))
	}
}

/// Return the stack index of the `n`-th upvalue, starting from `0`.
/// 
/// This function is not part of the public C++ API.
/// It is based on LuaJIT, and may break at any time
/// (though, it is unlikely).
#[inline]
pub const fn upvalue_index(n: u8) -> c_int {
	const LUA_GLOBALSINDEX: c_int = -10002;
	(LUA_GLOBALSINDEX - 1) - (n as c_int)
}

/// Wrapper type for integer-based references to Lua objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LuaRef(pub c_int);

/// Type for an error that has occurred in a Lua protected call.
// TODO: Handle Lua status codes?
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallError;
impl Error for CallError {}
impl fmt::Display for CallError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("error encountered in protected call")
	}
}
