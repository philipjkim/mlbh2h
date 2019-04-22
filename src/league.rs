use crate::utils;
use clap::ArgMatches;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub mod roster;
pub mod scoring;

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

    let league_dir = &format!("{}/.mlbh2h/leagues/{}", utils::get_home_dir(), league_name);
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

pub fn list_leagues() -> Result<(), Box<dyn Error>> {
    let base_dir = format!("{}/.mlbh2h/leagues", utils::get_home_dir());

    let subdirs = one_depth_subdirs(base_dir)?;

    println!("Available leagues:");
    subdirs.iter().for_each(|d| println!("\t{}", d));
    Ok(())
}

fn one_depth_subdirs(base_dir: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut subdirs: Vec<String> = Vec::new();

    let base_dir_len = base_dir.len();

    for entry in WalkDir::new(base_dir).min_depth(1).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path.to_str().unwrap()[base_dir_len + 1..].to_string());
        }
    }

    Ok(subdirs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_depth_subdirs_should_return_list_of_1depth_subdirectories() {
        let subdirs = one_depth_subdirs("src".to_string()).unwrap();
        println!("subdirs: {:?}", subdirs);
        assert_eq!(vec!["league".to_string(), "stats".to_string()], subdirs);
    }
}
