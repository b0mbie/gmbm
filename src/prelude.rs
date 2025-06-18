//! Often-used items for writing binary modules.

pub use crate::{
	gmod13::{
		cppdef::{
			CFunc as LuaCFunc,
			Special as LuaSpecial,
			Type as LuaType,
			StdType as LuaStdType,
		},
		Lua, LuaRef,
		upvalue_index as lua_upvalue_index,
		Module as LuaModule,
	},
	source::{
		Vector as SeVector,
		QAngle as SeQAngle,
	},
	gmod13_lua_function, gmod13_module,
};
