use gmbm::prelude::*;

pub enum Test {}
impl LuaModule for Test {
	fn open(lua: &mut Lua) {
		let mut lgc = lua.with_gc();
		lgc.push_globals();
		{
			lgc.create_table();

			lgc.push_c_function(gmod13_lua_function!(lua => {
				let Some(n_args) = lua.top().checked_sub(1) else {
					lua.arg_error(1, c"expected a value to pcall")
				};
				let is_ok = lua.with_gc().pcall(n_args, 0, 0).is_ok();
				lua.push_bool(is_ok);
				1
			}));
			lgc.set_field(-2, c"pcall");

			lgc.new_userdata(0);
			let udata_ty = lgc.get_type(-1);
			lgc.set_field(-2, c"proxy");

			lgc.push_number(udata_ty.0 as _);
			lgc.set_field(-2, c"proxy_ty");

			lgc.push_c_function(gmod13_lua_function!(_ => {
				panic!("Rust function panicked")
			}));
			lgc.set_field(-2, c"panic");

			lgc.push_c_function(gmod13_lua_function!(lua => {
				let value = lua.get_bool(1);
				lua.push_bool(value);
				1
			}));
			lgc.set_field(-2, c"to_bool");

			lgc.push_c_function(gmod13_lua_function!(lua => {
				let Some(value) = lua.get_string(1) else {
					return 0
				};
				lua.with_no_gc().push_string(value);
				1
			}));
			lgc.set_field(-2, c"to_string");
		}
		lgc.set_field(-2, c"test");
	}
}
gmod13_module!(Test);
