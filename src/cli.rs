use std::fmt::Display;

use clap::{App, SubCommand, Arg, ArgMatches};
use failure::ResultExt;

use crate::{config, Result, Client};

struct Cli<'a, F, R> where
    F: Fn(&str) -> R,
    R: Display,
{
    event: &'a str,
    day: u8,
    level: u8,
    code: F,
}

impl<'a, F, R> Cli<'a, F, R> where
    F: Fn(&str) -> R,
    R: Display,
{
    fn new(event: &'a str, day: u8, level: u8, code: F) -> Self {
        Self {
            event,
            day,
            level,
            code,
        }
    }

    fn run(&self) -> Result<()> {
        let cli = App::new(format!("Advent of Code {} - Day {} part {}", self.event, self.day, self.level))
            .subcommand(SubCommand::with_name("submit")
                .about("Submit the solution")
            )
            .subcommand(SubCommand::with_name("open")
                .about("Open the day's page in the browser")
            )
            .subcommand(new_config_subcommand())
            .get_matches();

        match cli.subcommand() {
            ("submit", _) => self.submit(),
            ("open", _) => self.open(),
            (CONFIG_SUBCOMMAND, Some(args)) => run_config_subcommand(args),
            _ => self.default(),
        }
    }

    fn default(&self) -> Result<()> {
        let client = Client::new(self.event, config::session_token()?)?;
        let input = client.get_input(self.day)?;
        let result = (self.code)(&input);

        println!("Result: '{}'", result);

        Ok(())
    }

    fn submit(&self) -> Result<()> {
        let client = Client::new(self.event, config::session_token()?)?;
        let input = client.get_input(self.day)?;
        let result = (self.code)(&input).to_string();

        println!("Submitting '{}' for AoC {} day {} part {}\n", result, self.event, self.day, self.level);

        let response = client.submit_solution(self.day, self.level, &result)?;

        println!("{}", response);

        Ok(())
    }

    fn open(&self) -> Result<()> {
        let url = format!("https://adventofcode.com/{}/day/{}", self.event, self.day);
        ::open::that(&url).with_context(|_| format!("Failed to open '{}'", url))?;
        Ok(())
    }
}

pub const CONFIG_SUBCOMMAND: &str = "config";

pub fn new_config_subcommand() -> App<'static, 'static> {
    SubCommand::with_name(CONFIG_SUBCOMMAND)
        .about("Configure advent of code settings")
        .arg(Arg::with_name("session")
            .short("s")
            .long("session")
            .help("Set the session token / cookie")
            .value_name("TOKEN")
            .takes_value(true)
        )
}

pub fn run_config_subcommand(args: &ArgMatches) -> Result<()> {
    if let Some(token) = args.value_of("session") {
        config::set_session_token(token)?;
    }

    Ok(())
}

pub fn run<F, R>(event: &str, day: u8, level: u8, code: F) where
    F: Fn(&str) -> R,
    R: Display,
{
    if let Err(error) = Cli::new(event, day, level, code).run() {
        println!("Error: {}", error.as_fail());
        for cause in error.iter_chain().skip(1) {
            println!("caused by: {}", cause);
        }
    }
}
