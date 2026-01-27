use gmbm::prelude::*;

// rust_hello.Add
extern "C-unwind" fn lua_add(cx: LuaCtx<'_>) -> LuaRets {
	let lua = cx.lua();
	let a = lua.check_number(1);
	let b = lua.check_number(2);
	lua.push_number(a + b);
	1.into()
}

impl LuaModule for Hello {
	fn open(&mut self, lua: &mut Lua) {
		lua.push_globals();
		{
			lua.create_table(); // rust_hello
			{
				lua.push_function(lua_add);
				lua.set_field(-2, c"Add");
			}
		}
		lua.set_field(-2, c"rust_hello");
	}
}

struct Hello;
gmod13_module!(Hello = Hello);
