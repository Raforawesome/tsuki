//! API module to test and debug the Lua/Rust bridge.
//! Used only to debug framework internals, holds no use for
//! other users.

use mlua::prelude::*;

/// The function which creates the Lua module table.
pub fn tsuki_dbg_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("print_hello", lua.create_function(print_hello)?)?;
    Ok(exports)
}

/// Used to verify if mlua is working.
/// ```lua
/// tsuki_dbg.print_hello()  -- should print "Hello from Rust"
/// ```
pub fn print_hello(_: &Lua, _: ()) -> LuaResult<()> {
    println!("Hello from Rust");
    Ok(())
}
