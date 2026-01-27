//! Items for implementing Garry's Mod Binary Modules which use `gmod13_*` entrypoints.

mod bits;
pub use bits::*;
mod raw;
pub use raw::*;
mod lua;
pub use lua::*;
mod types;
pub use types::*;

pub mod func;

#[cfg(feature = "user-types")]
pub mod user_types;

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
/// the given expression which implements [`Module`].
/// 
/// # Examples
/// ```
/// use gmbm::prelude::*;
/// 
/// struct Hello;
/// impl LuaModule for Hello {
///     fn open(&mut self, lua: &mut Lua) {
///         lua.push_globals();
///         lua.push_string("Hello, Garry's Mod!");
///         lua.set_field(-2, c"GREETING");
///     }
/// }
/// 
/// gmod13_module_with!(&mut Hello);
/// ```
#[macro_export]
macro_rules! gmod13_module_with {
	($($module:tt)+) => {
		const _: () = {
			#[unsafe(export_name = "gmod13_open")]
			unsafe extern "C-unwind" fn gmod13_open(
				state: *mut $crate::gmod13::LuaState,
			) -> ::core::ffi::c_int {
				let lua = unsafe { $crate::gmod13::Lua::from_mut_ptr(state) };
				$crate::gmod13::Module::open($($module)+, lua);
				0
			}

			#[unsafe(export_name = "gmod13_close")]
			unsafe extern "C-unwind" fn gmod13_close(
				state: *mut $crate::gmod13::LuaState,
			) -> ::core::ffi::c_int {
				let lua = unsafe { $crate::gmod13::Lua::from_mut_ptr(state) };
				$crate::gmod13::Module::close($($module)+, lua);
				0
			}
		};
	};

	() => {
		::core::compile_error! {
			"expected expression that evaluates to a value which implements `Module`"
		}
	};
}

/// Exports `gmod13_*` C++ entrypoint functions that redirect to
/// the given `static mut` item
/// the type of which implements [`Module`].
#[macro_export]
macro_rules! gmod13_module_static {
	($STATIC:ident) => {
		$crate::gmod13_module_with!(
			unsafe { &mut *&raw mut $STATIC }
		);
	};

	($($whatever:tt)*) => {
		::core::compile_error! {
			"expected name of `static mut` item to export"
		}
	};
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
///     fn open(&mut self, lua: &mut Lua) {
///         lua.push_globals();
///         lua.push_string("Hello, Garry's Mod!");
///         lua.set_field(-2, c"GREETING");
///     }
/// }
/// 
/// gmod13_module!(Hello = Hello);
/// ```
#[macro_export]
macro_rules! gmod13_module {
	{$Module:ty = $init:expr} => {
		const _: () = {
			static mut EXPORTED_GMOD13_MODULE: $Module = $init;
			// SAFETY: `gmod13_*` functions are always called from a single thread.
			$crate::gmod13_module_static!(EXPORTED_GMOD13_MODULE);
		};
	};

	($($whatever:tt)*) => {
		::core::compile_error! {
			"expected `<ModuleType> = <init expression>`"
		}
	};
}
