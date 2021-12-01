use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::fs::{self, File};

use chrono::prelude::*;
use directories::ProjectDirs;
use failure::ResultExt;
use serde_json as json;

use crate::{Result, Leaderboard};

#[derive(Serialize,Deserialize,Default)]
struct Config {
    leaderboard_url: Option<String>,
    session_token: String,
    last_api_access: Option<DateTime<Local>>,
    last_api_response: Option<Leaderboard>,
}

impl Config {
    fn load() -> Result<Self> {
        let path = config_path()?;
        let file = match File::open(path) {
            Ok(file) => file,
            Err(ref err) if err.kind() == NotFound => return Ok(Self::default()),
            Err(err) => return Err(err.into()),
        };
        let config = json::from_reader(file)?;

        Ok(config)
    }

    fn save(&self) -> Result<()> {
        let path = config_path()?;
        let file = File::create(path)?;
        json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

fn config_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("", "panicbit", "advent_of_code")
    .ok_or_else(|| format_err!("Failed to find config folder"))?;
    let dir = dirs.config_dir();

    fs::create_dir_all(dir)?;

    Ok(dir.join("config"))
}

pub fn leaderboard_url() -> Result<String> {
    let config = Config::load()?;
    let url = config.leaderboard_url.ok_or_else(|| format_err!(
        "Leaderboard url not set.\n\
        Set one using `--url https://adventofcode.com/YEAR/leaderboard/private/view/ID`.\n\
        You can get this URL by viewing your private leaderboard\n\
        and copying it from your browser's address bar."
    ))?;
    Ok(url)
}

pub fn set_leaderboard_url<U: Into<String>>(url: U) -> Result<()> {
    let mut config = Config::load()?;
    let mut url = url.into();

    if !url.ends_with(".json") {
        url += ".json";
    }

    config.leaderboard_url = Some(url);
    config.save().context("Failed to save leaderboard URL")?;

    Ok(())
}

pub fn session_token() -> Result<String> {
    let config = Config::load()?;
    let token = config.session_token;
    Ok(token)
}

pub fn set_session_token<S: Into<String>>(token: S) -> Result<()> {
    let mut config = Config::load()?;

    config.session_token = token.into();
    config.save().context("Failed to save session token")?;

    Ok(())
}

pub fn last_api_access() -> Result<Option<DateTime<Local>>> {
    let config = Config::load()?;
    Ok(config.last_api_access)
}

pub fn set_last_api_access(last_access: Option<DateTime<Local>>) -> Result<()> {
    let mut config = Config::load()?;

    config.last_api_access = last_access;
    config.save().context("Failed to save last API access timestamp")?;

    Ok(())
}

pub fn last_leaderboard() -> Result<Option<Leaderboard>> {
    let config = Config::load()?;
    Ok(config.last_api_response)
}

pub fn set_last_leaderboard(leaderboard: Leaderboard) -> Result<()> {
    let mut config = Config::load()?;

    config.last_api_response = Some(leaderboard);
    config.save().context("Failed to save last leaderboard")?;

    Ok(())
}
