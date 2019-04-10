extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod league;
mod version;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_app().get_matches();

    if let Some(m) = matches.subcommand_matches("new-league") {
        return league::add_new_league(m);
    }

    Ok(())
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    let version = version::get();

    App::new("mlbh2h")
        .version(version)
        .author("Soo Philip Jason Kim <philipjkim@gmail.com>")
        .about(
            "This app Shows Yahoo! Baseball Head-to-Head fantasy points by your scoring settings.",
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
