use clap::{crate_version, App, Arg, SubCommand};
use env_logger;
use log::error;
use std::error::Error;

mod league;
mod stats;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = get_app().get_matches();

    if let Some(m) = matches.subcommand_matches("new-league") {
        if let Err(e) = league::add_new_league(m) {
            error!("{}", e);
            return Err(e);
        }
        return Ok(());
    }

    if matches.subcommand_matches("list-leagues").is_some() {
        if let Err(e) = league::list_leagues() {
            error!("{}", e);
            return Err(e);
        }
        return Ok(());
    }

    if let Err(e) = stats::show(&matches) {
        error!("{}", e);
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
                .default_value(utils::yesterday_str()),
        )
        .arg(
            Arg::with_name("range")
                .short("r")
                .long("range")
                .value_name("RANGE")
                .help("Sets the range for stats (1d, 1w, 2w, 1m, all)")
                .takes_value(true)
                .default_value("1d"),
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
                .value_name("SPORTRADAR_API_KEY")
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
            Arg::with_name("topn")
                .short("t")
                .long("topn")
                .help("If set, top 10 * (number of t's) batters/pitchers are shown separately (-ttt for top 30 batters/pitchers)")
                .takes_value(false)
                .multiple(true),
        )
        .arg(
            Arg::with_name("weekly-changes")
            .short("w")
            .long("weekly-changes")
            .help("If set, fantasy points per team for 7 days are shown")
            .takes_value(false)
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
