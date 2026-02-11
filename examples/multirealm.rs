use gmbm::prelude::*;

impl LuaModule for Multirealm {
	fn open(&mut self, lua: &mut Lua) {
		lua.push_globals();
		lua.push_bool(self.loaded);
		self.loaded = true;
		lua.set_field(-2, c"RUST_MULTIREALM_WAS_LOADED");
	}
}

struct Multirealm {
	loaded: bool,
}

gmod13_module! {
	Multirealm = Multirealm {
		loaded: false,
	}
}
