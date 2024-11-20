//! A Lua framework akin to Node.JS, but for Lua.
//! Features provided are:
//! - [ ] fs
//! - [ ] http
//! - [ ] os
//! - [ ] window

mod api;

use api::tsuki_dbg::tsuki_dbg_module;
use clap::Parser;
use mlua::prelude::*;
use std::path::PathBuf;
use tsuki::Args;

fn main() -> LuaResult<()> {
    let args: Args = Args::parse();

    let mut parsed_path: PathBuf = PathBuf::from(args.file);
    if parsed_path.is_relative() {
        parsed_path = std::env::current_dir()
            .expect("Failed to read current directory!")
            .join(parsed_path);
    }

    let lua_file: String = std::fs::read_to_string(&parsed_path).expect("Failed to read Lua file!");

    let lua = Lua::new();
    lua.globals().set("tsuki_dbg", tsuki_dbg_module(&lua)?)?;
    lua.load(&lua_file).exec()?;

    Ok(())
}
