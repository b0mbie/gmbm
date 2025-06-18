use gmbm::prelude::*;

pub enum Test {}
impl LuaModule for Test {
	fn open(mut lua: Lua<'_>) {
		lua.push_globals();
		{
			lua.create_table();

			lua.push_c_function(gmod13_lua_function!(mut lua => {
				let Some(n_args) = lua.top().checked_sub(1) else {
					lua.arg_error(1, c"expected a value to pcall")
				};
				let is_ok = lua.pcall(n_args, 0, 0).is_ok();
				lua.push_bool(is_ok);
				1
			}));
			lua.set_field(-2, c"pcall");

			lua.new_userdata(0);
			let udata_ty = lua.get_type(-1);
			lua.set_field(-2, c"proxy");

			lua.push_number(udata_ty.0 as _);
			lua.set_field(-2, c"proxy_ty");

			lua.push_c_function(gmod13_lua_function!(_ => {
				panic!("Rust function panicked")
			}));
			lua.set_field(-2, c"panic");

		}
		lua.set_field(-2, c"test");
	}
}
gmod13_module!(Test);
