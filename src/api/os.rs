//! A module to interact with and provide
//! information on the OS/platform

use mlua::prelude::*;

/// The function which creates the Lua module table.
pub fn os_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("GET_OS_NAME", lua.create_function(os_name)?)?;
    exports.set("GET_ARCH_NAME", lua.create_function(arch_name)?)?;
    exports.set("GET_OS_FAMILY_NAME", lua.create_function(os_family_name)?)?;
    Ok(exports)
}

pub fn os_name(_: &Lua, _: ()) -> LuaResult<&'static str> {
    Ok(std::env::consts::OS)
}

pub fn arch_name(_: &Lua, _: ()) -> LuaResult<&'static str> {
    Ok(std::env::consts::ARCH)
}

pub fn os_family_name(_: &Lua, _: ()) -> LuaResult<&'static str> {
    Ok(std::env::consts::FAMILY)
}
