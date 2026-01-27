use gmbm::prelude::*;

impl LuaModule for UserTypes {
	fn open(&mut self, lua: &mut Lua) {
		lua.register::<MyType>();

		lua.push_globals();
		{
			lua.create_table();
			{
				lua.push_function(gmod13_fn!(lua => {
					let x = lua.check_number(1);
					let y = lua.check_number(2);
					let ty = lua.user_type_of::<MyType>();
					unsafe { lua.push_user_type(ty, MyType { x, y, }) };
					1
				}));
				lua.set_field(-2, c"MyType");
			}
			{
				lua.push_function(gmod13_fn!(lua => {
					lua.push_string(format!("{:?}", lua.get_userdata(1)));
					1
				}));
				lua.set_field(-2, c"Get");
			}
		}
		lua.set_field(-2, c"rust_user_types");
	}
}

gmod13_type!(MyType);
struct MyType {
	pub x: LuaNumber,
	pub y: LuaNumber,
}
impl Drop for MyType {
	fn drop(&mut self) {}
}
impl LuaUserType for MyType {
	fn init_metatable(mut cx: LuaSelfCtx<'_, Self>) {
		cx.push_method(gmod13_method!(MyType => mut lua => {
			let this = lua.check_self();
			let s = format!("MyType {{ x: {}, y: {} }}", this.x, this.y);
			lua.push_string(s);
			1
		}));
		cx.set_field(-2, c"__tostring");
	}

	unsafe fn collect(&mut self, mut cx: LuaSelfCtx<'_, Self>) {
		cx.push_globals();
		cx.get_field(-1, c"print");
		
		// print
		cx.push_string("MyType is being collected:");
		cx.push_number(self.x);
		cx.push_number(self.y);
		cx.call(3, 0);
	}
}

struct UserTypes;
gmod13_module!(UserTypes = UserTypes);
