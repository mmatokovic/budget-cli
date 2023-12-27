//! Help text for bud.
//!
//! Can contain information about what the binary does, command-line options,
//! configuration, etc.

// modules declarations
mod cli;
mod utils;
mod config;
mod database;

// This is the only export from the crate. It is marked hidden and
// is not part of the public API.
#[doc(hidden)]
pub use cli::Bud;
pub use config::Settings;
pub use database::Database;