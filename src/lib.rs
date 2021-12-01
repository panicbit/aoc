#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

pub use aoc_codegen::aoc;

pub use self::leaderboard::Leaderboard;
pub use self::client::Client;

pub mod cli;
pub mod leaderboard;
pub mod config;
pub mod client;
pub mod util;

pub type Result<T> = ::std::result::Result<T, failure::Error>;
