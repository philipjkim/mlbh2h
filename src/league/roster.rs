use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub fn add(dir: &str) -> Result<WeeklyRoster, Box<dyn Error>> {
    let filepath = &format!("{}/roster.json", dir);

    let roster = get_roster_from_stdin()?;

    fs::write(filepath, serde_json::to_string(&roster)?)?;
    println!("Saved weekly roster to {}.", filepath);

    Ok(roster)
}

fn get_roster_from_stdin() -> Result<WeeklyRoster, Box<dyn Error>> {
    let mut roster: WeeklyRoster = Default::default();

    let num_batters = get_usize_stdin("How many batters are in a team roster?", 1, 15)?;
    let num_pitchers = get_usize_stdin("How many pitchers are in a team roster?", 1, 15)?;
    println!("batters: {}, pitchers: {}", num_batters, num_pitchers);

    let num_teams = get_usize_stdin("How many teams are in your fantasy league?", 4, 12)?;
    let mut team_names: Vec<String> = Vec::new();
    while team_names.len() < num_teams as usize {
        if let Ok(name) =
            get_string_stdin(format!("Enter the name of team {} > ", team_names.len() + 1).as_str())
        {
            team_names.push(name);
        }
    }
    println!("team_names: {:#?}", team_names);

    for team in team_names {
        let mut batters = get_players_stdin(FantasyPlayerRole::Batter, num_batters, team.clone());
        roster.players.append(&mut batters);

        let mut pitchers =
            get_players_stdin(FantasyPlayerRole::Pitcher, num_pitchers, team.clone());
        roster.players.append(&mut pitchers);
    }

    Ok(roster)
}

fn get_players_stdin(role: FantasyPlayerRole, size: usize, team: String) -> Vec<FantasyPlayer> {
    let mut players: Vec<FantasyPlayer> = Vec::new();
    let mut players_saved = 0;
    while players_saved < size {
        let label = format!(
            "Enter the name of {:?} {} for team {} (ex: John Doe) > ",
            role,
            players_saved + 1,
            team
        );
        if let Ok(name) = get_string_stdin(label.as_str()) {
            players.push(FantasyPlayer {
                player_name: name,
                team: team.clone(),
                role: role.clone(),
            });
            players_saved += 1;
        }
    }

    players
}

fn get_usize_stdin(label: &str, min: usize, max: usize) -> Result<usize, Box<dyn Error>> {
    loop {
        let mut input_str = String::new();
        print!("{} ({}-{}) > ", label, min, max);
        io::stdout().flush()?;
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input, please retry.");
            continue;
        }

        match input_str.trim().parse::<usize>() {
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
        print!("{}", label);
        io::stdout().flush()?;
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input, please retry.");
            continue;
        }

        let s = input_str.trim();
        if s.len() >= 1 {
            return Ok(s.to_owned());
        }

        println!("Please input a non-empty string.");
    }
}
