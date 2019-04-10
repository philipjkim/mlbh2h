extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod league;
mod stats;
mod version;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_app().get_matches();

    if let Some(m) = matches.subcommand_matches("new-league") {
        if let Err(e) = league::add_new_league(m) {
            println!("{}", e);
            return Err(e);
        }
        return Ok(());
    }

    if let Err(e) = stats::show(&matches) {
        println!("{}", e);
        return Err(e);
    }

    Ok(())
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    let version = version::get();

    App::new("mlbh2h")
        .version(version)
        .author("Soo Philip Jason Kim <philipjkim@gmail.com>")
        .about("This app Shows Fantasy Baseball Head-to-Head points by your scoring settings.")
        .arg(
            Arg::with_name("date")
                .short("d")
                .long("date")
                .value_name("YYYY-MM-DD")
                .help("Sets the date for stats")
                .takes_value(true)
                .default_value("2019-04-01"),
        )
        .arg(
            Arg::with_name("league")
                .short("l")
                .long("league")
                .value_name("LEAGUE_NAME")
                .help("Sets the league name for scoring and roster")
                .takes_value(true)
                .default_value("sample"),
        )
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("apikey")
                .value_name("SPORTSRADAR_API_KEY")
                .help(
                    "Sets sportsradar API key.
Get a free api key at https://developer.sportradar.com/
if you don't have one yet.
Environment variable `SPORTRADAR_API_KEY` should be set
if you don't want to set this option.
The option value precedes env.",
                )
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("new-league")
                .about("adds a new league settings (scoring rules + rosters)")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .value_name("NAME")
                        .help("set the name of a new league")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("remove existing league settings if exists")
                        .takes_value(false),
                ),
        )
}
