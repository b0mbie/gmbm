# gmbm
Rust crate that allows one to write Garry's Mod Binary Modules in Rust without linking to LuaJIT at run-time,
instead only using the API that Garry's Mod itself provides.

## Prior art
The first crate to provide an interface to Garry's Mod was
[`WilliamVenner/gmod-rs`](https://github.com/WilliamVenner/gmod-rs).
It searches for and loads the `lua_shared` dynamic library that comes with Garry's Mod in order to interface with the
LuaJIT state,
however, after examining
[`Facepunch/gmod-module-base` (`development`)](https://github.com/Facepunch/gmod-module-base/tree/development),
I realized that there is a C++ API that is, seemingly, version-agnostic!
There are some valid reasons why it's not used, though:
- the C++ API is accessed via a C++ object with `virtual` methods, which can be super unsafe in Rust;
- there are quite a lot of API functions missing, and there is an
[issue on GitHub](https://github.com/Facepunch/gmod-module-base/issues/6) about it,
but it has seemingly not seen any activity from the developers.

Still, I thought that this would be an interesting project.
FWIW,
it allows one to write a `no_std` binary which doesn't try to load a dynamic library at some arbitrary path,
and there aren't too many functions missing.
You should probably consider using the aforementioned crate for actual projects, though!
