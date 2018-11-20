extern crate reqwest;
extern crate cachedir;
extern crate serde_json as json;
extern crate clap;
extern crate select;
extern crate serde;
extern crate chrono;
extern crate chrono_tz;
extern crate open;
extern crate directories;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate aoc_codegen;

pub use aoc_codegen::aoc;

pub use self::leaderboard::Leaderboard;
pub use self::client::Client;

pub mod cli;
pub mod leaderboard;
pub mod config;
pub mod client;
pub mod util;

pub type Result<T> = ::std::result::Result<T, failure::Error>;
