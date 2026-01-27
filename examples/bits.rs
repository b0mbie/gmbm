use gmbm::prelude::*;

impl LuaModule for Bits {
	fn open(&mut self, lua: &mut Lua) {
		lua.push_globals();
		{
			lua.create_table();
			{
				lua.push_function(gmod13_fn!(lua => {
					let f = lua.check_number(1);
					lua.push_bits(f as _);
					1
				}));
				lua.set_field(-2, c"ToBits");
			}
			{
				lua.push_function(gmod13_fn!(lua => {
					let b = lua.check_bits(1);
					lua.push_number(b as _);
					1
				}));
				lua.set_field(-2, c"FromBits");
			}
		}
		lua.set_field(-2, c"rust_bits");
	}
}

struct Bits;
gmod13_module!(Bits = Bits);
