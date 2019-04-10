use clap::ArgMatches;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

mod roster;
mod scoring;

#[derive(Debug, Clone)]
pub struct LeagueNameConflict(pub String);
impl fmt::Display for LeagueNameConflict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "league with name {} already exists, use other name",
            self.0
        )
    }
}
impl Error for LeagueNameConflict {}

#[derive(Debug, Clone)]
pub struct InvalidLeagueName;
impl fmt::Display for InvalidLeagueName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "league name should not start with _")
    }
}
impl Error for InvalidLeagueName {}

pub fn add_new_league(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let league_name = matches.value_of("name").unwrap();
    if league_name.starts_with("_") {
        return Err(Box::new(InvalidLeagueName));
    }

    let league_dir = &format!("data/{}", league_name);
    if Path::new(league_dir).exists() {
        match matches.occurrences_of("force") {
            0 => return Err(Box::new(LeagueNameConflict(league_name.to_string()))),
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
