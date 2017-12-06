use std::fmt::Display;
use {Result, Client};
use clap::{App,SubCommand};

pub fn run<F, R>(event: &str, day: u8, level: u8, code: F) where
    F: Fn(&str) -> R,
    R: Display,
{
    let cli = App::new(format!("Advent of Code {} - Day {} part {}", event, day, level))
        .subcommand(SubCommand::with_name("submit")
            .about("Submit the solution")
        )
        .get_matches();
    
    match cli.subcommand() {
        ("submit", _) => submit(event, day, level, code).unwrap(),
        _ => default(event, day, code).unwrap(),
    }
}

fn default<F, R>(event: &str, day: u8, code: F) -> Result<()> where
    F: Fn(&str) -> R,
    R: Display,
{
    let input = client(event)?.get_input(day)?;
    let result = code(&input);

    println!("Result: '{}'", result);

    Ok(())
}

fn submit<F, R>(event: &str, day: u8, level: u8, code: F) -> Result<()> where
    F: Fn(&str) -> R,
    R: Display,
{
    let client = client(event)?;
    let input = client.get_input(day)?;
    let result = code(&input).to_string();

    println!("Submitting '{}' for AoC {} day {} part {}", result, event, day, level);

    client.submit_solution(day, level, &result)?;

    Ok(())
}

fn client(event: &str) -> Result<Client> {
    let session_token = ::get_session_token()?;
    Client::new(event.to_string(), session_token)
}
