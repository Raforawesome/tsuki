//! A Lua framework akin to Node.js, but for Lua.
//! Features provided are:
//! - [ ] fs
//! - [ ] http
//! - [ ] os
//! - [ ] window

use clap::Parser;
use mlua::prelude::*;
use std::path::PathBuf;
use tsuki::{
    api::{fs::fs_module, tsuki_dbg::tsuki_dbg_module},
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

    let lua = Lua::new();
    lua.globals()
        .set("TSUKI_DBG_INTERNAL", tsuki_dbg_module(&lua)?)?;
    lua.globals().set("FS_INTERNAL", fs_module(&lua)?)?;
    lua.load(&lua_file).exec()?;

    Ok(())
}
