extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

mod roster;
mod scoring;
mod version;

#[derive(Debug, Clone)]
pub struct LeagueNameConflictError(pub String);
impl fmt::Display for LeagueNameConflictError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "league with name {} already exists, use other name",
            self.0
        )
    }
}
impl Error for LeagueNameConflictError {}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_app().get_matches();

    if let Some(m) = matches.subcommand_matches("new-league") {
        let league_name = m.value_of("name").unwrap();
        let league_dir = &format!("data/{}", league_name);
        if Path::new(league_dir).exists() {
            match m.occurrences_of("force") {
                0 => return Err(Box::new(LeagueNameConflictError(league_name.to_string()))),
                _ => fs::remove_dir_all(league_dir)?,
            }
        }
        fs::create_dir_all(league_dir)?;

        let rule = scoring::add(league_dir);
        match rule {
            Ok(r) => {
                println!("Saved the scoring: {:#?}", r);
            }
            Err(e) => return Err(e),
        }

        let roster = roster::add(league_dir);
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
