The Garry's Mod C++ API is a simple wrapper around the LuaJIT C API,
which uses the equivalents to the C library's `setjmp` and `longjmp` functions.
These functions unwind the stack in a similar fashion to Rust's panicking system.

In practice, this means that any Lua errors that occur in an API call will cease the execution of the current function,
which may require special attention when arbitrary Lua functions are ran from Rust.
In this case, the binary module may be required to make *protected* calls to Lua,
through the usage of the `pcall` method.
