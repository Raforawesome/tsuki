//! A Lua framework akin to Node.js, but for Lua.
//! Features provided are:
//! - [X] fs
//! - [ ] http
//! - [ ] os
//! - [ ] window

use clap::Parser;
use mlua::prelude::*;
use std::path::PathBuf;
use tsuki::{
    api::{fs::fs_module, os::os_module, tsuki_dbg::tsuki_dbg_module},
    Args,
};

fn main() -> LuaResult<()> {
    tsuki::module_setup::populate_modules()?;

    let args: Args = Args::parse();

    let mut parsed_path: PathBuf = PathBuf::from(args.file);
    if parsed_path.is_relative() {
        parsed_path = std::env::current_dir()
            .expect("Failed to read current directory!")
            .join(parsed_path);
    }

    let lua_file: String = std::fs::read_to_string(&parsed_path).expect("Failed to read Lua file!");

    std::env::set_var(
        "LUA_PATH",
        format!(
            "{}/?.lua;;",
            tsuki::module_setup::get_module_dir().to_str().unwrap()
        ),
    );

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("TSUKI_DBG_INTERNAL", tsuki_dbg_module(&lua)?)?;
    globals.set("FS_INTERNAL", fs_module(&lua)?)?;
    globals.set("OS_INTERNAL", os_module(&lua)?)?;
    lua.load(&lua_file).exec()?;

    Ok(())
}
