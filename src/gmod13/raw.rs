//! C++ definitions for Garry's Mod's Lua API.

use core::{
	ffi::{
		c_int, c_char, c_uint, c_void, c_double,
	},
	ptr::NonNull,
};
use cpp_class::class;

use crate::source::{
	Vector, QAngle,
};

/// Special value in the Lua state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum Special {
	/// Global table.
	Glob,
	/// Environment table.
	Env,
	/// Registry table.
	Registry,
}

/// LuaJIT state structure provided by
/// the same Garry's Mod version that uses `gmod13_open` and `gmod13_close` functions for binary modules.
#[derive(Debug)]
#[repr(C)]
pub struct LuaState {
	#[cfg(target_pointer_width = "32")]
	_ignore_this_common_lua_header: [u8; 48 + 22],
	#[cfg(target_pointer_width = "64")]
	_ignore_this_common_lua_header: [u8; 92 + 22],
	// I still don't understand why this field even exists in the first place.
	// The entrypoints could just have this object as an additional argument.
	// Especially considering that `SetState` exists, and these entrypoints are special in some way.
	// INVARIANT: The pointer is valid as a C++ object with a virtual function table.
	pub luabase: NonNull<LuaBase>,
}

/// Type of C (or native) functions that can be executed by Lua.
pub type CFunc = unsafe extern "C-unwind" fn (state: *mut LuaState) -> c_int;

class! {
	/// Virtual function table for the C++ API provided by Garry's Mod.
	/// 
	/// Based on `ILuaBase` in `GarrysMod/Lua/LuaBase.h`.
	pub LuaBase {
		pub virtual fn top() -> c_int;
		pub virtual fn push(stack_pos: StackPos);
		pub virtual fn pop(amt: c_int);
		pub virtual fn get_table(stack_pos: StackPos);
		pub virtual fn get_field(stack_pos: StackPos, name: *const c_char);
		pub virtual fn set_field(stack_pos: StackPos, name: *const c_char);
		pub virtual fn create_table();
		pub virtual fn set_table(i: StackPos);
		pub virtual fn set_meta_table(i: StackPos);
		pub virtual fn get_meta_table(i: StackPos) -> bool;
		pub virtual fn call(n_args: c_int, n_results: c_int);
		pub virtual fn pcall(n_args: c_int, n_results: c_int, error_func: c_int) -> c_int;
		pub virtual fn equal(a: StackPos, b: StackPos) -> c_int;
		pub virtual fn raw_equal(a: StackPos, b: StackPos) -> c_int;
		pub virtual fn insert(stack_pos: StackPos);
		pub virtual fn remove(stack_pos: StackPos);
		pub virtual fn next(stack_pos: StackPos) -> c_int;
		pub virtual fn new_userdata(size: c_uint) -> *mut c_void;
		pub virtual fn throw_error(error: *const c_char) -> !;
		pub virtual fn check_type(stack_pos: StackPos, ty: RawType);
		pub virtual fn arg_error(arg_num: c_int, message: *const c_char) -> !;
		pub virtual fn raw_get(stack_pos: StackPos);
		pub virtual fn raw_set(stack_pos: StackPos);
		pub virtual fn get_string(stack_pos: StackPos, out_len: *mut c_uint) -> *const c_char;
		pub virtual fn get_number(stack_pos: StackPos) -> Number;
		pub virtual fn get_bool(stack_pos: StackPos) -> bool;
		pub virtual fn get_c_function(stack_pos: StackPos) -> Option<CFunc>;
		pub virtual fn get_userdata(stack_pos: StackPos) -> *mut c_void;
		pub virtual fn push_nil();
		pub virtual fn push_string(val: *const c_char, len: c_int);
		pub virtual fn push_number(val: Number);
		pub virtual fn push_bool(val: bool);
		pub virtual fn push_c_function(val: CFunc);
		pub virtual fn push_c_closure(val: CFunc, n_upvalues: c_int);
		pub virtual fn push_userdata(val: *mut c_void);
		pub virtual fn reference_create() -> RawRef;
		pub virtual fn reference_free(i: RawRef);
		pub virtual fn reference_push(i: RawRef);
		pub virtual fn push_special(special: c_int);
		pub virtual fn is_type(stack_pos: StackPos, ty: RawType) -> bool;
		pub virtual fn get_type(stack_pos: StackPos) -> RawType;
		pub virtual fn get_type_name(ty: RawType) -> *const c_char;
		pub virtual fn create_meta_table_type(name: *const c_char, ty: RawType);
		pub virtual fn check_string(stack_pos: StackPos) -> *const c_char;
		pub virtual fn check_number(stack_pos: StackPos) -> Number;
		pub virtual fn obj_len(stack_pos: StackPos) -> c_int;
		pub virtual fn get_angle(stack_pos: StackPos) -> NonNull<QAngle>;
		pub virtual fn get_vector(stack_pos: StackPos) -> NonNull<Vector>;
		pub virtual fn push_angle(val: *const QAngle);
		pub virtual fn push_vector(val: *const Vector);
		pub virtual fn set_state(l: *mut LuaState);
		pub virtual fn create_meta_table(name: *const c_char) -> c_int;
		pub virtual fn push_meta_table(ty: RawType) -> bool;
		pub virtual fn push_user_type(data: *mut c_void, ty: RawType);
		pub virtual fn set_user_type(stack_pos: StackPos, data: *mut c_void);
	}
}

/// Type for references to values in the Lua state.
pub type RawRef = c_int;

/// Type for positions on the Lua stack.
pub type StackPos = c_int;

/// Floating-point number type.
pub type Number = c_double;

/// Integer type used internally to identify Lua types.
pub type RawType = c_int;
