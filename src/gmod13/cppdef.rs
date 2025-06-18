//! C++ definitions for Garry's Mod's Lua API.

use core::{
	ffi::{
		c_int, c_char, c_uint, c_void, c_double,
	},
	ptr::NonNull,
};
use cppdvt::{
	VtObject, vtable,
};

use crate::source::{
	Vector, QAngle,
};

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
	luabase: VtObject<LuaBaseVt>,
}

impl LuaState {
	/// Return the raw pointer representing the Lua API that this structure targets.
	/// 
	/// See [`Lua`](super::lua::Lua).
	#[inline]
	pub const fn api_ptr(&self) -> VtObject<LuaBaseVt> {
		self.luabase
	}
}

/// Type of C (or native) functions that can be executed by Lua.
pub type CFunc = unsafe extern "C-unwind" fn (state: *mut LuaState) -> c_int;

vtable! {
	/// Virtual function table for the C++ API provided by Garry's Mod.
	/// 
	/// Based on `ILuaBase` in `GarrysMod/Lua/LuaBase.h`.
	pub LuaBaseVt {
		pub fn top() -> c_int;
		pub fn push(stack_pos: c_int);
		pub fn pop(amt: c_int);
		pub fn get_table(stack_pos: c_int);
		pub fn get_field(stack_pos: c_int, name: *const c_char);
		pub fn set_field(stack_pos: c_int, name: *const c_char);
		pub fn create_table();
		pub fn set_table(i: c_int);
		pub fn set_meta_table(i: c_int);
		pub fn get_meta_table(i: c_int) -> bool;
		pub fn call(n_args: c_int, n_results: c_int);
		pub fn pcall(n_args: c_int, n_results: c_int, error_func: c_int) -> c_int;
		pub fn equal(a: c_int, b: c_int) -> c_int;
		pub fn raw_equal(a: c_int, b: c_int) -> c_int;
		pub fn insert(stack_pos: c_int);
		pub fn remove(stack_pos: c_int);
		pub fn next(stack_pos: c_int) -> c_int;
		pub fn new_userdata(size: c_uint) -> *mut c_void;
		pub fn throw_error(error: *const c_char) -> !;
		pub fn check_type(stack_pos: c_int, ty: c_int);
		pub fn arg_error(arg_num: c_int, message: *const c_char) -> !;
		pub fn raw_get(stack_pos: c_int);
		pub fn raw_set(stack_pos: c_int);
		pub fn get_string(stack_pos: c_int, out_len: *mut c_uint) -> *const c_char;
		pub fn get_number(stack_pos: c_int) -> c_double;
		pub fn get_bool(stack_pos: c_int) -> bool;
		pub fn get_c_function(stack_pos: c_int) -> CFunc;
		pub fn get_userdata(stack_pos: c_int) -> *mut c_void;
		pub fn push_nil();
		pub fn push_string(val: *const c_char, len: c_int);
		pub fn push_number(val: c_double);
		pub fn push_bool(val: bool);
		pub fn push_c_function(val: CFunc);
		pub fn push_c_closure(val: CFunc, n_upvalues: c_int);
		pub fn push_userdata(val: *mut c_void);
		pub fn reference_create() -> c_int;
		pub fn reference_free(i: c_int);
		pub fn reference_push(i: c_int);
		pub fn push_special(special: c_int);
		pub fn is_type(stack_pos: c_int, ty: c_int) -> bool;
		pub fn get_type(stack_pos: c_int) -> c_int;
		pub fn get_type_name(ty: c_int) -> *const c_char;
		fn _create_meta_table_type(name: *const c_char, ty: c_int);
		pub fn check_string(stack_pos: c_int) -> *const c_char;
		pub fn check_number(stack_pos: c_int) -> c_double;
		pub fn obj_len(stack_pos: c_int) -> c_int;
		pub fn get_angle(stack_pos: c_int) -> NonNull<QAngle>;
		pub fn get_vector(stack_pos: c_int) -> NonNull<Vector>;
		pub fn push_angle(val: *const QAngle);
		pub fn push_vector(val: *const Vector);
		pub fn set_state(l: *mut LuaState);
		pub fn create_meta_table(name: *const c_char) -> c_int;
		pub fn push_meta_table(ty: c_int) -> bool;
		pub fn push_user_type(data: *mut c_void, ty: c_int);
		pub fn set_user_type(stack_pos: c_int, data: *mut c_void);
	}
}

/// Wrapper for types returned by the Garry's Mod Lua API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Type(pub c_int);

impl Type {
	/// Return a [`Type`] that represents the specified [`StdType`].
	pub const fn from_std(ty: StdType) -> Self {
		Self(ty as c_int)
	}

	/// Return `true` if this type is the specified [`StdType`].
	pub const fn is_std(&self, ty: StdType) -> bool {
		self.0 == (ty as c_int)
	}
}

impl From<StdType> for Type {
	fn from(value: StdType) -> Self {
		Self::from_std(value)
	}
}

impl PartialEq<StdType> for Type {
	fn eq(&self, other: &StdType) -> bool {
		self.is_std(*other)
	}
}
impl PartialEq<Type> for StdType {
	fn eq(&self, other: &Type) -> bool {
		other.is_std(*self)
	}
}

/// Enumeration of pre-defined types in Garry's Mod Lua.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum StdType {
	None = -1,
	Nil,
	Bool,
	LightUserData,
	Number,
	String,
	Table,
	Function,
	UserData,
	Thread,

	// GMod types.
	Entity,
	Vector,
	Angle,
	PhysObj,
	Save,
	Restore,
	DamageInfo,
	EffectData,
	MoveData,
	RecipientFilter,
	UserCmd,
	ScriptedVehicle,
	Material,
	Panel,
	Particle,
	ParticleEmitter,
	Texture,
	UserMsg,
	ConVar,
	IMesh,
	Matrix,
	Sound,
	PixelVisHandle,
	DLight,
	Video,
	File,
	Locomotion,
	Path,
	NavArea,
	SoundHandle,
	NavLadder,
	ParticleSystem,
	ProjectedTexture,
	PhysCollide,
	SurfaceInfo,
}
