//! Macros for writing shell scripts in Rust.

mod arg;
mod cmd;
mod cmdline;
mod redirects;
mod script;

pub use arg::Arg;
pub use cmd::Cmd;
pub use cmdline::Cmdline;
pub use redirects::{Stderr, Stdin, Stdout};
pub use script::Script;
pub use shelle_macros::{eval, exec};
