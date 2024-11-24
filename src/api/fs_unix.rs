//! A module to interact with the filesystem of the
//! machine the code is being run on.

use mlua::prelude::*;

/// The function which creates the Lua module table.
pub fn fs_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("raw_path_exists", lua.create_function(raw_path_exists)?)?;
    exports.set("split_path", lua.create_function(split_path)?)?;
    Ok(exports)
}

/// Lua API function to call [PathBuf::exists] on a [String] `path`.
pub fn raw_path_exists(_: &Lua, path: String) -> LuaResult<bool> {
    Ok(std::path::PathBuf::from(path).exists())
}

pub fn split_path(_: &Lua, path: String) -> LuaResult<Vec<String>> {
    let mut buffer: Vec<String> = vec![];
    let mut buf_cur: String = String::new();
    let mut normalized_path: String;

    // Path normalization
    #[allow(clippy::redundant_pattern_matching)]
    if let Some(_) = path.strip_prefix("/") {
        // Case: Absolute path provided
        // current behaviour is to do nothing as we normalize
        // all paths into absolute paths in this step.
        normalized_path = path;
    } else if let Some(path) = path.strip_prefix("../") {
        let mut cd = std::env::current_dir().unwrap();
        if cd.is_file() {
            cd.pop();
        }
        cd.pop();
        normalized_path = cd
            .to_str()
            .ok_or(LuaError::runtime("Bad path! Check for unicode."))?
            .to_owned();
        normalized_path.push('/');
        normalized_path.push_str(path);
    } else {
        normalized_path = std::env::current_dir()
            .unwrap()
            .to_str()
            .ok_or(LuaError::runtime("Bad path! Check for unicode."))?
            .to_owned();
        normalized_path.push('/');
        if let Some(path) = path.strip_prefix("./") {
            normalized_path.push_str(path);
        } else {
            normalized_path.push_str(&path);
        }
    }

    // Separate normalized path
    let mut prev: char = 0 as char;
    for c in normalized_path.chars().skip(1) {
        if c == '/' && prev != '\\' {
            buffer.push(buf_cur);
            buf_cur = String::new();
        } else if c != '\\' || prev == '\\' {
            buf_cur.push(c);
        }
        prev = c;
    }
    buffer.push(buf_cur);

    Ok(buffer)
}
