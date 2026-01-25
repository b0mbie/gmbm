//! Items for implementing Garry's Mod Binary Modules which use `gmod13_*` entrypoints.

pub mod cppdef;
mod lua;
pub use lua::*;

/// Returns a [`CFunc`](cppdef::CFunc) that can be used by Lua,
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
			state: *mut $crate::gmod13::cppdef::LuaState,
		) -> ::core::ffi::c_int {
			let $lua = unsafe { $crate::gmod13::Lua::from_mut_ptr(state) };
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
	fn open(&mut self, lua: &mut Lua);

	/// Function called when the binary module is unloaded.
	// TODO: Clarify when exactly a binary module is unloaded!
	fn close(&mut self, lua: &mut Lua) {
		let _ = lua;
	}
}

/// Exports `gmod13_*` C++ entrypoint functions that redirect to
/// the given type which implements [`Module`].
/// 
/// # Examples
/// ```
/// use gmbm::prelude::*;
/// 
/// struct Hello;
/// impl LuaModule for Hello {
/// 	fn open(&mut self, lua: &mut Lua) {
/// 		lua.push_globals();
/// 		let mut lgc = lua.with_gc();
/// 		lgc.push_string("Hello, Garry's Mod!");
/// 		lgc.set_field(-2, c"GREETING");
/// 	}
/// }
/// 
/// gmod13_module!(Hello = Hello);
/// ```
#[macro_export]
macro_rules! gmod13_module {
	($Module:ty = $init:expr) => {
		const _: () = {
			static mut EXPORTED_GMOD13_MODULE: $Module = $init;
			// SAFETY: `gmod13_*` functions are always called from a single thread.
			$crate::gmod13_module_from!(unsafe { &mut *::core::ptr::addr_of_mut!(EXPORTED_GMOD13_MODULE) });
		};
	};
}

/// Exports `gmod13_*` C++ entrypoint functions that redirect to
/// the given expression which implements [`Module`].
/// 
/// # Examples
/// ```
/// use gmbm::prelude::*;
/// 
/// struct Hello;
/// impl LuaModule for Hello {
/// 	fn open(&mut self, lua: &mut Lua) {
/// 		lua.push_globals();
/// 		let mut lgc = lua.with_gc();
/// 		lgc.push_string("Hello, Garry's Mod!");
/// 		lgc.set_field(-2, c"GREETING");
/// 	}
/// }
/// 
/// gmod13_module_from!(&mut Hello);
/// ```
#[macro_export]
macro_rules! gmod13_module_from {
	($module:expr) => {
		const _: () = {
			#[unsafe(export_name = "gmod13_open")]
			unsafe extern "C-unwind" fn gmod13_open(
				state: *mut $crate::gmod13::cppdef::LuaState,
			) -> ::core::ffi::c_int {
				let lua = unsafe { $crate::gmod13::Lua::from_mut_ptr(state) };
				$crate::gmod13::Module::open($module, lua);
				0
			}
	
			#[unsafe(export_name = "gmod13_close")]
			unsafe extern "C-unwind" fn gmod13_close(
				state: *mut $crate::gmod13::cppdef::LuaState,
			) -> ::core::ffi::c_int {
				let lua = unsafe { $crate::gmod13::Lua::from_mut_ptr(state) };
				$crate::gmod13::Module::close($module, lua);
				0
			}
		};
	};
}
