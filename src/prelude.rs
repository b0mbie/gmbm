//! Often-used items for writing binary modules.

pub use crate::{
	gmod13::{
		func::{
			Ctx as LuaCtx, Rets as LuaRets,
		},
		Special as LuaSpecial,
		Type as LuaType,
		StdType as LuaStdType,
		Lua, Ref,
		Number as LuaNumber,
		Bits as LuaBits,
		upvalue_index as lua_upvalue_index,
		Module as LuaModule,
	},
	source::{
		Vector as SeVector,
		QAngle as SeQAngle,
	},
	gmod13_fn,
	gmod13_module, gmod13_module_with, gmod13_module_static,
	gmod13_type,
};

#[cfg(feature = "user-types")]
pub use crate::{
	gmod13::user_types::{
		UserType as LuaUserType,
		SelfCtx as LuaSelfCtx,
	},
	gmod13_method,
};
