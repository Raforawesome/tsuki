//! A Lua framework akin to Node.js, but for Lua.

#![feature(os_string_pathbuf_leak)]

pub mod api;
pub mod module_setup;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// A relative path to the Lua file being run.
    pub file: String,
}
