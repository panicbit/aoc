#[macro_use]
extern crate serde_derive;

pub use aoc_codegen::aoc;

pub use self::client::Client;
pub use self::leaderboard::Leaderboard;

pub mod cli;
pub mod client;
pub mod config;
pub mod leaderboard;
pub mod util;

pub type Result<T> = anyhow::Result<T>;
