extern crate reqwest;
extern crate cachedir;
extern crate preferences;
extern crate clap;
extern crate select;
extern crate serde;
extern crate chrono;
extern crate chrono_tz;
extern crate open;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

use preferences::AppInfo;

pub use self::leaderboard::Leaderboard;
pub use self::client::Client;

pub mod cli;
pub mod leaderboard;
pub mod config;
pub mod client;
pub mod util;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

const APP_INFO: &AppInfo = &AppInfo {
    name: "advent_of_code",
    author: "panicbit"
};
