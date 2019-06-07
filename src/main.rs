use clap::{crate_version, App, Arg, SubCommand};
use std::error::Error;

mod league;
mod stats;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_app().get_matches();

    if let Some(m) = matches.subcommand_matches("new-league") {
        if let Err(e) = league::add_new_league(m) {
            println!("{}", e);
            return Err(e);
        }
        return Ok(());
    }

    if matches.subcommand_matches("list-leagues").is_some() {
        if let Err(e) = league::list_leagues() {
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
    App::new("mlbh2h")
        .version(&crate_version!()[..])
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
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Sets the output format, available values: pretty, csv")
                .takes_value(true)
                .default_value("pretty"),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("If set, all FA players are also shown")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("top10")
                .short("t")
                .long("top10")
                .help("If set, top 10 batters/pitchers are shown separately")
                .takes_value(false),
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
        .subcommand(SubCommand::with_name("list-leagues").about("lists previously added leagues"))
}
