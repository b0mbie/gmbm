//! Items for implementing Garry's Mod Binary Modules which use `gmod13_*` entrypoints.

pub mod cppdef;
mod lua;
pub use lua::*;

/// Return a [`CFunc`](cppdef::CFunc) that can be used by Lua,
/// given a function definition similar to a Rust closure.
/// 
/// # Examples
/// ```
/// use gmbm::prelude::*;
/// 
/// let func: LuaCFunc = gmod13_lua_function!(mut lua => {
/// 	lua.with_gc().push_string("Hey every    !");
/// 	1
/// });
/// ```
#[macro_export]
macro_rules! gmod13_lua_function {
	($lua:pat => $body:tt) => {{
		#[allow(unsafe_op_in_unsafe_fn)]
		unsafe extern "C-unwind" fn __gmod13_lua_function(
			state: *mut ::gmbm::gmod13::cppdef::LuaState,
		) -> ::core::ffi::c_int {
			let $lua = unsafe { ::gmbm::gmod13::Lua::from_ptr_mut((*state).api_ptr()) };
			$body
		}
		__gmod13_lua_function
	}};
}

/// Trait for binary modules that can be loaded by Garry's Mod.
// TODO: Is there a better way to express this?
// Using Rust modules for this would be confusing since it would require a structure defined in prose.
pub trait Module {
	/// Function called when the binary module is first loaded.
	fn open(lua: &mut Lua);

	/// Function called when the binary module is unloaded.
	// TODO: Clarify when exactly a binary module is unloaded!
	fn close(lua: &mut Lua) {
		let _ = lua;
	}
}

/// Export `gmod13_*` C++ entrypoint functions that redirect to the given type implementing [`Module`].
/// 
/// # Examples
/// ```
/// use gmbm::prelude::*;
/// 
/// enum Hello {}
/// impl LuaModule for Hello {
/// 	fn open(lua: &mut Lua) {
/// 		lua.push_globals();
/// 		let mut lgc = lua.with_gc();
/// 		lgc.push_string("Hello, Garry's Mod!");
/// 		lgc.set_field(-2, c"GREETING");
/// 	}
/// }
/// 
/// gmod13_module!(Hello);
/// ```
#[macro_export]
macro_rules! gmod13_module {
	($module:ty) => {
		const _: () = {
			#[unsafe(export_name = "gmod13_open")]
			pub unsafe extern "C-unwind" fn gmod13_open(
				state: *mut ::gmbm::gmod13::cppdef::LuaState,
			) -> ::core::ffi::c_int {
				let lua = unsafe { ::gmbm::gmod13::Lua::from_ptr_mut((*state).api_ptr()) };
				unsafe { <$module as ::gmbm::gmod13::Module>::open(lua); }
				0
			}
	
			#[unsafe(export_name = "gmod13_close")]
			pub unsafe extern "C-unwind" fn gmod13_close(
				state: *mut ::gmbm::gmod13::cppdef::LuaState,
			) -> ::core::ffi::c_int {
				let lua = unsafe { ::gmbm::gmod13::Lua::from_ptr_mut((*state).api_ptr()) };
				unsafe { <$module as ::gmbm::gmod13::Module>::close(lua); }
				0
			}
		};
	};
}
