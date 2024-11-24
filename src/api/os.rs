//! A module to interact with and provide
//! information on the OS/platform

use mlua::prelude::*;

/// The function which creates the Lua module table.
pub fn fs_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    Ok(exports)
}

pub fn get_os(_: &Lua) -> LuaResult<&'static str> {
    Ok(std::env::consts::OS)
}

pub fn get_arch(_: &Lua) -> LuaResult<&'static str> {
    Ok(std::env::consts::ARCH)
}

pub fn get_os_family(_: &Lua) -> LuaResult<&'static str> {
    Ok(std::env::consts::FAMILY)
}
