use core::{
	cell::UnsafeCell,
	error::Error,
	ffi::{
		CStr,
		c_int, c_uint, c_void,
	},
	fmt,
	mem::MaybeUninit,
	ops::{
		Deref, DerefMut,
	},
	ptr::null_mut,
	slice::{
		from_raw_parts as slice_from_raw_parts,
		from_raw_parts_mut as slice_from_raw_parts_mut,
	},
};
use cpp_class::virtual_call;

use crate::source::{
	Vector, QAngle,
};

use super::func::*;
use super::*;

/// Interface for the Lua environment of
/// the same Garry's Mod version that uses `gmod13_open` and `gmod13_close` functions for binary modules.
#[derive(Debug)]
#[repr(transparent)]
pub struct Lua {
	luabase: UnsafeCell<LuaBase>,
}

impl Lua {
	/// Returns a mutable reference to the Lua state provided by Garry's Mod.
	/// 
	/// # Safety
    /// `ptr` must be a valid Lua state from the Garry's Mod version this structure targets.
	pub const unsafe fn from_mut_ptr<'a>(ptr: *mut LuaState) -> &'a mut Self {
		unsafe { Self::from_luabase_mut((*ptr).luabase.as_mut()) }
	}

	/// See [`LuaState`].
	/// 
	/// # Safety
    /// `object` must be a valid Lua state from the Garry's Mod version this structure targets.
	const unsafe fn from_luabase(luabase: &LuaBase) -> &Self {
		unsafe { &*(luabase as *const _ as *const _) }
	}

	/// See [`LuaState`].
	/// 
	/// # Safety
    /// `object` must be a valid Lua state from the Garry's Mod version this structure targets.
	const unsafe fn from_luabase_mut(luabase: &mut LuaBase) -> &mut Self {
		unsafe { &mut *(luabase as *mut _ as *mut _) }
	}

	unsafe fn with_luabase_mut<'a, F: FnOnce(&'a mut LuaBase) -> R, R>(&'a self, f: F) -> R {
		unsafe {
			let luabase = &mut *self.luabase.get();
			f(luabase)
		}
	}

	unsafe fn with_luabase<'a, F: FnOnce(&'a LuaBase) -> R, R>(&'a self, f: F) -> R {
		unsafe {
			let luabase = &*self.luabase.get();
			f(luabase)
		}
	}

	/// Returns the amount of values on the stack.
	pub fn top(&self) -> c_uint {
		unsafe { self.with_luabase(move |l| virtual_call!(l => top()) as _) }
	}

	/// Pushes a copy of the value at `stack_pos` to the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_value(&self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push(stack_pos))) }
	}

	/// Pops `n` values from the stack.
	pub fn pop(&self, amt: c_uint) {
		// SAFETY: To-be-closed slots aren't a thing in Lua 5.1 and LuaJIT.
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => pop(amt as _))) }
	}

	/// Sets the metatable for the value at `stack_pos` to the value popped from the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_metatable(&self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => set_meta_table(stack_pos))) }
	}

	/// Pushes the metatable of the value at `stack_pos` on the stack, returning `true`,
	/// or pushes nothing and returns `false` on failure.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn get_metatable(&self, stack_pos: StackPos) -> bool {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => get_meta_table(stack_pos))) }
	}

	/// Returns `true` if the values at `a` and `b` are equal.
	/// 
	/// See also [`Lua::raw_equal`].
	pub fn equal(&self, a: StackPos, b: StackPos) -> bool {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => equal(a, b)) != 0) }
	}

	/// Returns `true` if the values at `a` and `b` are equal,
	/// without invoking metamethods.
	/// 
	/// See also [`Lua::equal`].
	pub fn raw_equal(&self, a: StackPos, b: StackPos) -> bool {
		unsafe { self.with_luabase(move |l| virtual_call!(l => raw_equal(a, b)) != 0) }
	}
	
	/// Moves the value at the top of the stack into `stack_pos`,
	/// shifting upwards any elements above `stack_pos`.
	pub fn insert(&self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => insert(stack_pos))) }
	}

	/// Removes the value at `stack_pos`,
	/// shifting values above `stack_pos` downwards.
	pub fn remove(&self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => remove(stack_pos))) }
	}

	/// Throws an error and ceases execution of the function.
	pub fn throw_error(&self, message: &'static CStr) -> ! {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => throw_error(message.as_ptr()))) }
	}

	/// Throws an error if the value at `stack_pos` is not of the given [`Type`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_type<Ty: Into<Type>>(&self, stack_pos: StackPos, ty: Ty) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => check_type(stack_pos, ty.into().0))) }
	}

	/// Throws an error related to argument `arg_num` and cease execution of the function.
	pub fn arg_error(&self, arg_num: c_int, message: &'static CStr) -> ! {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => arg_error(arg_num, message.as_ptr()))) }
	}

	/// Without metamethods, pushes the value of `t[key]`, where
	/// `t` is the value at the given index,
	/// and `key` is the value popped from the stack.
	pub fn raw_get(&self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => raw_get(stack_pos))) }
	}

	/// Without metamethods, does `t[key] = value`, where
	/// `t` is the value at the given index,
	/// `value` is the value popped from the stack,
	/// and `key` is the value just below the top.
	pub fn raw_set(&self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => raw_set(stack_pos))) }
	}

	/// Returns the contents of the Lua string at `stack_pos`,
	/// converting any Lua number at that position to a string in the process,
	/// and returns `None` if the value can't be converted to a Lua string.
	pub fn get_string(&self, stack_pos: StackPos) -> Option<&[u8]> {
		let mut len = MaybeUninit::uninit();
		let string_ptr = unsafe {
			self.with_luabase_mut(move |l| virtual_call!(l => get_string(stack_pos, len.as_mut_ptr())))
		};
		if !string_ptr.is_null() {
			// SAFETY: If `string_ptr` isn't null, then it should be valid for reads, and `len` should be initialized.
			unsafe { Some(slice_from_raw_parts(string_ptr as *const _, len.assume_init() as _)) }
		} else {
			None
		}
	}

	/// Returns the Lua C string at `stack_pos`,
	/// converting any Lua number at that position to a string in the process,
	/// and returns `None` if the value can't be converted to a Lua string.
	pub fn get_c_string(&self, stack_pos: StackPos) -> Option<&CStr> {
		let string_ptr = unsafe { self.with_luabase_mut(move |l| virtual_call!(l => get_string(stack_pos, null_mut()))) };
		if !string_ptr.is_null() {
			// SAFETY: If `string_ptr` isn't null, then it should be a valid C string.
			unsafe { Some(CStr::from_ptr(string_ptr)) }
		} else {
			None
		}
	}

	/// Returns the [`Number`] at `stack_pos`,
	/// or `0.0` if the value isn't a Lua number.
	pub fn get_number(&self, stack_pos: StackPos) -> Number {
		unsafe { self.with_luabase(move |l| virtual_call!(l => get_number(stack_pos))) }
	}

	/// Returns `true` if the value at `stack_pos` is truthy.
	pub fn get_bool(&self, stack_pos: StackPos) -> bool {
		unsafe { self.with_luabase(move |l| virtual_call!(l => get_bool(stack_pos))) } 
	}

	/// Returns the [`CFunc`] at `stack_pos`,
	/// or a null pointer if the value isn't a C function.
	pub fn get_c_function(&self, stack_pos: StackPos) -> Option<CFunc> {
		unsafe { self.with_luabase(move |l| virtual_call!(l => get_c_function(stack_pos))) }
	}

	/// Returns the non-null pointer to the userdata at `stack_pos`,
	/// or a null pointer if the value isn't userdata.
	pub fn get_userdata(&self, stack_pos: StackPos) -> *mut c_void {
		unsafe { self.with_luabase(move |l| virtual_call!(l => get_userdata(stack_pos))) }
	}

	/// Pushes a `nil` onto the stack as a Lua string.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_nil(&self) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_nil())) }
	}

	/// Pushes the given [`Number`] onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_number(&self, n: Number) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_number(n))) }
	}

	/// Pushes the given boolean onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_bool(&self, b: bool) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_bool(b))) }
	}

	/// Pushes the given C function onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_c_function(&self, func: CFunc) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_c_function(func))) }
	}

	/// Pushes the light userdata `ptr` onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	/// 
	/// # Safety
	/// `ptr` is a plain pointer value that can be accessed at any later point in time,
	/// and so the exact guarantees for `ptr` vary depending on the use-case.
	/// Consider using full userdata instead if you can.
	pub unsafe fn push_light_userdata<T>(&self, ptr: *mut T) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_userdata(ptr as *mut _))) }
	}

	/// Frees the reference `lua_ref` if it is valid.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn free_ref(&self, lua_ref: Ref) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => reference_free(lua_ref.0))) }
	}

	/// Pushes the value pointed to by `lua_ref` onto the stack,
	/// or `nil` if the reference is invalid.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_ref(&self, lua_ref: Ref) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => reference_push(lua_ref.0))) }
	}

	/// Pushes a [`Special`] value onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_special(&self, what: Special) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_special(what as _))) }
	}

	/// Returns `true` if the value at `stack_pos` is of the given [`Type`].
	pub fn is_type<Ty: Into<Type>>(&self, stack_pos: StackPos, ty: Ty) -> bool {
		unsafe { self.with_luabase(move |l| virtual_call!(l => is_type(stack_pos, ty.into().0))) }
	}

	/// Returns the [`Type`] of the value at `stack_pos`.
	pub fn get_type(&self, stack_pos: StackPos) -> Type {
		unsafe { Type(self.with_luabase(move |l| virtual_call!(l => get_type(stack_pos)))) }
	}
	
	/// Returns the name of the given [`StdType`], as a C string.
	pub fn get_type_name(&self, ty: StdType) -> &CStr {
		unsafe { CStr::from_ptr(self.with_luabase(move |l| virtual_call!(l => get_type_name(ty as _)))) }
	}

	/// If the value at `stack_pos` is a string, returns it.
	/// Otherwise, throws an error.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_string(&self, stack_pos: StackPos) -> &CStr {
		unsafe { CStr::from_ptr(self.with_luabase_mut(move |l| virtual_call!(l => check_string(stack_pos)))) }
	}

	/// If the value at `stack_pos` is a [`Number`], returns it.
	/// Otherwise, throws an error.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn check_number(&self, stack_pos: StackPos) -> Number {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => check_number(stack_pos))) }
	}

	/// If the value at `stack_pos` is a [`QAngle`], returns a reference to it.
	/// Otherwise, returns a reference to the angle `0, 0, 0`.
	pub fn get_angle(&self, stack_pos: StackPos) -> &QAngle {
		unsafe { self.with_luabase(move |l| virtual_call!(l => get_angle(stack_pos)).as_ref()) }
	}

	/// If the value at `stack_pos` is a [`Vector`], returns a reference to it.
	/// Otherwise, returns a reference to the vector `0, 0, 0`.
	pub fn get_vector(&self, stack_pos: StackPos) -> &Vector {
		unsafe { self.with_luabase(move |l| virtual_call!(l => get_vector(stack_pos)).as_ref()) }
	}
	
	/// Pushes the metatable associated with the given [`Type`],
	/// returning `true` if it exists.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	// TODO: What happens when it doesn't exist?
	pub fn push_metatable<Ty: Into<Type>>(&self, ty: Ty) -> bool {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_meta_table(ty.into().0))) }
	}

	/// Pushes userdata of type `ty` referencing the data at `ptr`.
	/// 
	/// # Safety
	/// `ptr` must be valid for values of type `ty`.
	pub unsafe fn push_user_type_raw<T>(&self, ptr: *mut T, ty: Type) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_user_type(ptr as *mut _, ty.0))) }
	}

	/// Sets the data pointer of the userdata value at `stack_pos` to `ptr`.
	/// 
	/// # Safety
	/// `ptr` must be valid for values of type `ty`.
	pub unsafe fn set_user_type<T>(&self, stack_pos: StackPos, ptr: *mut T) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => set_user_type(stack_pos, ptr as *mut _))) }
	}

	/// Returns a context for operations on the Lua state
	/// that are asserted at compile time to
	/// not run the garbage collector
	/// and potentially invalidate any existing pointers returned by Lua.
	/// 
	/// # Safety
	/// Any existing Lua pointers must not be invalidated by the Lua state.
	/// This usually involves the garbage collector not running,
	/// but is also a valid assumption if the pointee is currently on the Lua stack.
	pub const unsafe fn with_no_gc(&self) -> WithNoGc<'_> {
		WithNoGc {
			luabase: unsafe { &mut *self.luabase.get() },
		}
	}
}

/// Functions which may cause the garbage collector to run and invalidate existing pointers.
impl Lua {
	/// Pushes the value `t[key]`,
	/// where `t` is the value at `stack_pos`,
	/// and `key` is the value popped from the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn get_table(&mut self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => get_table(stack_pos))) }
	}

	/// Pushes the value `t[key]`,
	/// where `t` is the value at `stack_pos`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn get_field(&mut self, stack_pos: StackPos, key: &CStr) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => get_field(stack_pos, key.as_ptr()))) }
	}

	/// Sets `t[key]` to the value popped from the stack,
	/// where `t` is the value at `stack_pos`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_field(&mut self, stack_pos: StackPos, key: &CStr) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => set_field(stack_pos, key.as_ptr()))) }
	}

	/// Creates a new table and pushes it onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn create_table(&mut self) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => create_table())) }
	}

	/// Sets `t[key]` to the value popped from the stack,
	/// where `t` is the value at `stack_pos`,
	/// and `key` is the value popped from just below the top of the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_table(&mut self, stack_pos: StackPos) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => set_table(stack_pos))) }
	}

	/// Calls an object as a function on the stack,
	/// propagating any error that occurred from the call.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn call(&mut self, n_args: c_uint, n_results: c_uint) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => call(n_args as _, n_results as _))) }
	}

	/// Calls an object as a function on the stack,
	/// returning `Err` if the function raised an error.
	pub fn pcall(&mut self, n_args: c_uint, n_results: c_int, error_func: c_int) -> Result<(), CallError> {
		let result = unsafe { self.with_luabase_mut(move |l| virtual_call!(l => pcall(n_args as _, n_results, error_func))) };
		if result == 0 {
			Ok(())
		} else {
			Err(CallError)
		}
	}

	/// Pushes the given non-empty slice of bytes onto the stack as a Lua string.
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
			self.with_luabase_mut(move |l| virtual_call!(l => push_string(
				non_empty_bytes.as_ptr() as *const _, non_empty_bytes.len() as _
			)))
		}
	}

	/// Pushes the given byte string onto the stack as a Lua string.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_string<S: AsRef<[u8]>>(&mut self, bytes: S) {
		let bytes_ref = bytes.as_ref();
		// If length is `0`, `strlen` is used. Which is very, very bad!
		if !bytes_ref.is_empty() {
			unsafe { self.push_non_empty_bytes(bytes_ref) }
		} else {
			unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_string(c"".as_ptr(), 0))) }
		}
	}

	/// Pushes the given C string onto the stack as a Lua string.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_c_string<S: AsRef<CStr>>(&mut self, string: S) {
		let c_string = string.as_ref();
		// TODO: Is it OK if `count_bytes` replaces the internal `strlen` calculation?
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_string(c_string.as_ptr() as *const _, c_string.count_bytes() as _))) }
	}

	/// Pushes the given C function onto the stack
	/// with `n_upvalues` to create a closure,
	/// which must be on the top of the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_c_closure(&mut self, func: CFunc, n_upvalues: u8) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_c_closure(func, n_upvalues as _))) }
	}

	/// Pops a value from the stack,
	/// and returns a [`Ref`] that can be used to access it later.
	/// 
	/// See also [`Lua::push_ref`] and [`Lua::free_ref`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn create_ref(&mut self) -> Ref {
		let index = unsafe { self.with_luabase_mut(move |l| virtual_call!(l => reference_create())) };
		Ref(index)
	}

	/// Returns the length of the object at `stack_pos`.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn length_of(&mut self, stack_pos: StackPos) -> c_int {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => obj_len(stack_pos))) }
	}

	/// Pushes `angle` onto the stack as a Lua object.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_angle(&mut self, angle: &QAngle) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_angle(angle))) }
	}

	/// Pushes `vector` onto the stack as a Lua object.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_vector(&mut self, vector: &Vector) {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => push_vector(vector))) }
	}

	/// Pushes the metatable associated with the given `name`,
	/// creating it if it doesn't exist,
	/// and return the [`Type`] to use for it.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn create_metatable(&mut self, name: &CStr) -> Type {
		unsafe { Type(self.with_luabase_mut(move |l| virtual_call!(l => create_meta_table(name.as_ptr())))) }
	}

	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	// TODO: Describe functionality.
	pub fn next(&mut self, stack_pos: StackPos) -> c_int {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => next(stack_pos))) }
	}

	/// Allocates a new Lua userdata of the specified `size`,
	/// returning the opaque pointer to it,
	/// which may be null if allocation failed.
	/// 
	/// # Safety
	/// Lua ensures that the pointer is valid as long as the corresponding userdata is alive.
	/// Moreover, if the userdata is marked for finalization,
	/// it is valid at least until the call to its finalizer.
	/// Do not use the returned pointer outside of these two specific circumstances!
	pub unsafe fn new_userdata_raw(&mut self, size: c_uint) -> *mut c_void {
		unsafe { self.with_luabase_mut(move |l| virtual_call!(l => new_userdata(size))) }
	}

	/// Allocates a new Lua userdata of the specified `size`,
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

/// Additional functions that are not part of the public C++ API.
impl Lua {
	/// Drains the stack so that it has *at most* a specific number of elements.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::pop`] and [`Lua::top`] for convenience.
	pub fn drain_to(&self, top: c_uint) {
		self.pop(self.top().saturating_sub(top))
	}

	/// Sets the number of elements in the stack,
	/// filling any excess slots with `nil`.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::pop`], [`Lua::top`] and [`Lua::push_nil`] for convenience.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn set_top(&self, top: c_uint) {
		let current_top = self.top();
		if let Some(to_push) = top.checked_sub(current_top) {
			for _ in 0..to_push {
				self.push_nil();
			}
		} else if let Some(to_pop) = current_top.checked_sub(top) {
			self.pop(to_pop)
		}
	}

	/// Pushes the globals table onto the stack.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::push_special`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_globals(&self) {
		self.push_special(Special::Glob)
	}

	/// Pushes the environment table onto the stack.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::push_special`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_environment(&self) {
		self.push_special(Special::Env)
	}

	/// Pushes the registry table onto the stack.
	/// 
	/// This method is not part of the public C++ API.
	/// It is implemented with [`Lua::push_special`].
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_registry(&self) {
		self.push_special(Special::Registry)
	}

	/// Pushes the `n`-th upvalue onto the stack, starting from `0`,
	/// or `nil` if the upvalue index is invalid.
	/// 
	/// This method is not part of the public C++ API.
	/// It is based on LuaJIT, and may break at any time
	/// (though, it is unlikely).
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	#[inline]
	pub fn push_upvalue(&self, n: u8) {
		self.push_value(upvalue_index(n))
	}

	/// Pushes the given function onto the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_function(&self, f: Func) {
		self.push_c_function(to_c_func(f))
	}

	/// Pushes the given function onto the stack
	/// with `n_upvalues` to create a closure,
	/// which must be on the top of the stack.
	/// 
	/// # Errors
	/// The inner Lua state may raise an [error](crate::errors).
	pub fn push_closure(&mut self, f: Func, n_upvalues: u8) {
		self.push_c_closure(to_c_func(f), n_upvalues)
	}

	/// Sets `t[i]` to the value popped from the stack,
	/// where `t` is the value at `stack_pos`.
	pub fn set_int(&mut self, stack_pos: StackPos, i: usize) {
		self.push_number(i as _);
		self.insert(-2);
		self.set_table(stack_pos.saturating_sub_unsigned(2));
	}
}

/// Returns the stack index of the `n`-th upvalue, starting from `0`.
/// 
/// This function is not part of the public C++ API.
/// It is based on LuaJIT, and may break at any time
/// (though, it is unlikely).
#[inline]
pub const fn upvalue_index(n: u8) -> c_int {
	const LUA_GLOBALSINDEX: c_int = -10002;
	(LUA_GLOBALSINDEX - 1) - (n as c_int)
}

/// Integer-based reference to a Lua object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Ref(pub RawRef);

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

/// Context for operations on [`Lua`]
/// which are asserted to not run the garbage collector
/// and invalidate existing pointers returned by Lua.
/// 
/// See [`Lua::with_no_gc`].
#[repr(transparent)]
pub struct WithNoGc<'a> {
	luabase: &'a mut LuaBase,
}

impl AsRef<Lua> for WithNoGc<'_> {
	fn as_ref(&self) -> &Lua {
		self.as_lua()
	}
}
impl AsMut<Lua> for WithNoGc<'_> {
	fn as_mut(&mut self) -> &mut Lua {
		self.as_lua_mut()
	}
}

impl Deref for WithNoGc<'_> {
	type Target = Lua;
	fn deref(&self) -> &Self::Target {
		self.as_lua()
	}
}
impl DerefMut for WithNoGc<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_lua_mut()
	}
}

impl WithNoGc<'_> {
	const fn as_lua(&self) -> &Lua {
		unsafe { Lua::from_luabase(self.luabase) }
	}

	const fn as_lua_mut(&mut self) -> &mut Lua {
		unsafe { Lua::from_luabase_mut(self.luabase) }
	}
}
