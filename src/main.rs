extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod roster;
mod scoring_rule;
mod version;

fn main() -> Result<(), Box<dyn Error>> {
    let app = get_app();

    if let Some(m) = app.get_matches().subcommand_matches("new-league") {
        let rule = scoring_rule::add(m.value_of("name").unwrap());
        match rule {
            Ok(r) => {
                println!("Saved the rule: {:#?}", r);
            }
            Err(e) => return Err(e),
        }

        let roster = roster::add(m.value_of("name").unwrap());
        match roster {
            Ok(r) => {
                println!("Saved the roster: {:#?}", r);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
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
                        .help("set the name of a new league")
                        .takes_value(true)
                        .required(true),
                ),
        )
}
