//! Module to set up ~/.tsuki/modules and populate it with
//! manually written Lua bindings to library (mostly for LSP purposes)

/// Directly embed all source files into binary because
/// text is small and I'm lazy
pub const TSUKI_DBG_BINDINGS: &str = include_str!("../src-lua/tsuki-dbg.lua");

use std::{
    cell::LazyCell,
    path::{Path, PathBuf},
};

thread_local! {
    /// Store module dir (pointless micro-optimization)
    pub static MODULE_DIR: LazyCell<&'static mut Path> = LazyCell::new(|| {
        let mut dir: PathBuf = dirs::home_dir().unwrap();
        dir.push(".tsuki");
        dir.push("modules");
        dir.leak()
    });
    // pub static MODULE_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    //     let mut dir: PathBuf = dirs::home_dir().unwrap();
    //     dir.push(".tsuki");
    //     dir.push("modules");
    //     dir
    // });
}

pub fn get_module_dir() -> PathBuf {
    let mut dir: PathBuf = dirs::home_dir().unwrap();
    dir.push(".tsuki");
    dir.push("modules");
    dir
}

pub fn populate_modules() -> std::io::Result<()> {
    let modules_dir: PathBuf = get_module_dir();
    let tsuki_dbg_path: PathBuf = modules_dir.join("tsuki_dbg.lua");

    if !tsuki_dbg_path.try_exists()? {
        std::fs::write(&tsuki_dbg_path, TSUKI_DBG_BINDINGS)?;
    }

    Ok(())
}
