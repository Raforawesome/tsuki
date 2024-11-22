//! A module to interact with the filesystem of the
//! machine the code is being run on.

use mlua::prelude::*;

/// The function which creates the Lua module table.
pub fn fs_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("raw_path_exists", lua.create_function(raw_path_exists)?)?;
    Ok(exports)
}

pub fn raw_path_exists(_: &Lua, path: String) -> LuaResult<bool> {
    Ok(std::path::PathBuf::from(path).exists())
}
