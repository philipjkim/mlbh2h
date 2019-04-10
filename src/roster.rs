use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;

use crate::scoring_rule::LeagueNameConflictError;

#[derive(Serialize, Deserialize, Debug)]
pub enum FantasyPlayerRole {
    Batter,
    Pitcher,
}
impl Default for FantasyPlayerRole {
    fn default() -> Self {
        FantasyPlayerRole::Batter
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FantasyPlayer {
    player_name: String,
    role: FantasyPlayerRole,
    team: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WeeklyRoster {
    players: Vec<FantasyPlayer>,
}

pub fn add(league_name: &str) -> Result<WeeklyRoster, Box<dyn Error>> {
    let league_dir = &format!("data/{}", league_name);
    fs::create_dir_all(league_dir)?;

    let filepath = &format!("{}/roster.json", league_dir);
    if Path::new(filepath).exists() {
        return Err(Box::new(LeagueNameConflictError(league_name.to_string())));
    }

    let roster = get_roster_from_stdin()?;

    fs::write(filepath, serde_json::to_string(&roster)?)?;
    println!("Saved scoring rule to {}.", filepath);

    Ok(roster)
}

fn get_roster_from_stdin() -> Result<WeeklyRoster, Box<dyn Error>> {
    let mut roster: WeeklyRoster = Default::default();

    let num_batters = get_number_stdin("How many batters are in a team roster?", 1, 15)?;
    let num_pitchers = get_number_stdin("How many pitchers are in a team roster?", 1, 15)?;
    println!("batters: {}, pitchers: {}", num_batters, num_pitchers);

    let num_teams = get_number_stdin("How many teams are in your fantasy league?", 4, 12)?;
    let mut team_names: Vec<String> = Vec::new();
    while team_names.len() < num_teams as usize {
        if let Ok(name) =
            get_string_stdin(format!("Enter the name of team {}> ", team_names.len() + 1).as_str())
        {
            team_names.push(name);
        }
    }
    println!("team_names: {:#?}", team_names);

    for team in team_names {
        let mut batters_saved = 0;
        while batters_saved < num_batters as usize {
            let label = format!(
                "Enter the name of batter {} for team {} (ex: Mike Trout) > ",
                batters_saved + 1,
                team
            );
            if let Ok(name) = get_string_stdin(label.as_str()) {
                roster.players.push(FantasyPlayer {
                    player_name: name,
                    team: team.clone(),
                    role: FantasyPlayerRole::Batter,
                });
                batters_saved += 1;
            }
        }

        let mut pitchers_saved = 0;
        while pitchers_saved < num_pitchers as usize {
            let label = format!(
                "Enter the name of pitcher {} for team {} (ex: Chris Sale) > ",
                pitchers_saved + 1,
                team
            );
            if let Ok(name) = get_string_stdin(label.as_str()) {
                roster.players.push(FantasyPlayer {
                    player_name: name,
                    team: team.clone(),
                    role: FantasyPlayerRole::Pitcher,
                });
                pitchers_saved += 1;
            }
        }
    }

    Ok(roster)
}

fn get_number_stdin(label: &str, min: i32, max: i32) -> Result<i32, Box<dyn Error>> {
    loop {
        let mut input_str = String::new();
        print!("{} ({}-{}) > ", label, min, max);
        io::stdout().flush()?;
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input, please retry.");
            continue;
        }

        match input_str.trim().parse::<i32>() {
            Ok(i) if i >= min && i <= max => {
                return Ok(i);
            }
            _ => println!("Please input a number between {} and {}.", min, max),
        }
    }
}

fn get_string_stdin(label: &str) -> Result<String, Box<dyn Error>> {
    loop {
        let mut input_str = String::new();
        print!("{} > ", label);
        io::stdout().flush()?;
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input, please retry.");
            continue;
        }

        let s = input_str.trim();
        if s.len() > 1 {
            return Ok(s.to_owned());
        }

        println!("Please input a non-empty string.");
    }
}
