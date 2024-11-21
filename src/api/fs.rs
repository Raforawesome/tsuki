//! A module to interact with the filesystem of the
//! machine code is being run on.

use mlua::prelude::*;

/// The function which creates the Lua module table.
pub fn fs_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    // exports.set("print_hello", lua.create_function(print_hello)?)?;
    Ok(exports)
}
