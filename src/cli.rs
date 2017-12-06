use std::fmt::Display;
use {Result, Client};
use clap::{App, SubCommand, Arg, ArgMatches};

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
            .subcommand(SubCommand::with_name("config")
                .about("Configure advent of code settings")
                .arg(Arg::with_name("session")
                    .short("s")
                    .long("session")
                    .help("Set the session token / cookie")
                    .value_name("TOKEN")
                    .takes_value(true)
                )
            )
            .get_matches();

        match cli.subcommand() {
            ("submit", _) => self.submit(),
            ("config", Some(args)) => self.config(args),
            _ => self.default(),
        }
    }

    fn default(&self) -> Result<()> {
        let client = Client::new(self.event, ::get_session_token()?)?;
        let input = client.get_input(self.day)?;
        let result = (self.code)(&input);

        println!("Result: '{}'", result);

        Ok(())
    }

    fn submit(&self) -> Result<()> {
        let client = Client::new(self.event, ::get_session_token()?)?;
        let input = client.get_input(self.day)?;
        let result = (self.code)(&input).to_string();

        println!("Submitting '{}' for AoC {} day {} part {}\n", result, self.event, self.day, self.level);

        let response = client.submit_solution(self.day, self.level, &result)?;

        println!("{}", response);

        Ok(())
    }

    fn config(&self, args: &ArgMatches) -> Result<()> {
        if let Some(token) = args.value_of("session") {
            ::set_session_token(token)?;
        }

        Ok(())
    }
}

pub fn run<F, R>(event: &str, day: u8, level: u8, code: F) where
    F: Fn(&str) -> R,
    R: Display,
{
    if let Err(error) = Cli::new(event, day, level, code).run() {
        println!("Error: {}", error.cause());
        for cause in error.causes().skip(1) {
            println!("caused by: {}", cause);
        }
    }
}

#[macro_export]
macro_rules! aoc {
    ($event:expr, $day:expr, $level:expr, |$input:ident| $code:expr) => {
        fn main() {
            $crate::cli::run(&$event.to_string(), $day, $level, |$input| $code);
        }
    }
}
