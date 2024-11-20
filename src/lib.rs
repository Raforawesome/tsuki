//! A Lua framework akin to Node.JS, but for Lua.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// A relative path to the Lua file being run.
    pub file: String,
}
