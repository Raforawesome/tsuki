//! Module to set up ~/.tsuki/modules and populate it with
//! manually written Lua bindings to library

/// Directly embed all source files into binary because
/// text is small and I'm lazy
pub const TSUKI_DBG_BINDINGS: &str = include_str!("../src-lua/tsuki-dbg.lua");
#[cfg(unix)]
pub const FS_BINDINGS: &str = include_str!("../src-lua/fs-unix.lua");
#[cfg(windows)]
pub const FS_BINDINGS: &str = include_str!("../src-lua/fs-windows.lua");
pub const OS_BINDINGS: &str = include_str!("../src-lua/os.lua");

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
        dir.push("tsuki");
        dir.leak()
    });
}

pub fn get_module_dir() -> PathBuf {
    let mut dir: PathBuf = dirs::home_dir().unwrap();
    dir.push(".tsuki");
    dir.push("modules");
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }
    dir
}

pub fn populate_modules() -> std::io::Result<()> {
    let modules_dir: PathBuf = get_module_dir().join("tsuki");
    std::fs::create_dir_all(&modules_dir)?;
    let tsuki_dbg_path: PathBuf = modules_dir.join("tsuki-dbg.lua");
    let fs_path: PathBuf = modules_dir.join("fs.lua");
    let sys_path: PathBuf = modules_dir.join("os.lua");

    std::fs::write(&tsuki_dbg_path, TSUKI_DBG_BINDINGS)?;
    std::fs::write(&fs_path, FS_BINDINGS)?;
    std::fs::write(&sys_path, OS_BINDINGS)?;

    Ok(())
}
