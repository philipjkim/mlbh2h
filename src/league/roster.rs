use crate::utils;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerType {
    Batter,
    Pitcher,
}
impl Default for PlayerType {
    fn default() -> Self {
        PlayerType::Batter
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Player {
    pub name: String,
    pub role: PlayerType,
    pub team: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Roster {
    pub players: Vec<Player>,
}

pub fn add(dir: &str) -> Result<Roster, Box<dyn Error>> {
    let filepath = &format!("{}/roster.json", dir);

    let roster = get_roster_from_stdin()?;

    fs::write(filepath, serde_json::to_string(&roster)?)?;
    println!("Saved weekly roster to {}.", filepath);

    Ok(roster)
}

fn get_roster_from_stdin() -> Result<Roster, Box<dyn Error>> {
    let mut roster: Roster = Default::default();

    let num_batters = get_usize_stdin("How many batters are in a team roster?", 1, 15)?;
    let num_pitchers = get_usize_stdin("How many pitchers are in a team roster?", 1, 15)?;
    println!("batters: {}, pitchers: {}", num_batters, num_pitchers);

    let num_teams = get_usize_stdin("How many teams are in your fantasy league?", 2, 12)?;
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
        let mut batters = get_players_stdin(PlayerType::Batter, num_batters, team.clone());
        roster.players.append(&mut batters);

        let mut pitchers = get_players_stdin(PlayerType::Pitcher, num_pitchers, team.clone());
        roster.players.append(&mut pitchers);
    }

    Ok(roster)
}

fn get_players_stdin(role: PlayerType, size: usize, team: String) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut players_saved = 0;
    while players_saved < size {
        let label = format!(
            "Enter the name of {:?} {} for team {} (ex: John Doe) > ",
            role,
            players_saved + 1,
            team
        );
        if let Ok(name) = get_string_stdin(label.as_str()) {
            players.push(Player {
                name: name,
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

pub fn load(league_name: &String) -> Result<Roster, Box<dyn Error>> {
    if league_name == "sample" {
        return Ok(sample_roster());
    }

    let filepath = format!(
        "{}/.mlbh2h/leagues/{}/roster.json",
        utils::get_home_dir(),
        league_name
    );
    println!("Loading the weekly roster from file {}", filepath);
    let json = fs::read_to_string(filepath)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn sample_roster() -> Roster {
    Roster {
        players: vec![
            Player {
                name: "Cody Bellinger".to_owned(),
                role: PlayerType::Batter,
                team: "LA Bulls".to_owned(),
            },
            Player {
                name: "Domingo Santana".to_owned(),
                role: PlayerType::Batter,
                team: "LA Bulls".to_owned(),
            },
            Player {
                name: "Blake Snell".to_owned(),
                role: PlayerType::Pitcher,
                team: "LA Bulls".to_owned(),
            },
            Player {
                name: "Max Scherzer".to_owned(),
                role: PlayerType::Pitcher,
                team: "LA Bulls".to_owned(),
            },
            Player {
                name: "Christian Yelich".to_owned(),
                role: PlayerType::Batter,
                team: "Chicago Pizzas".to_owned(),
            },
            Player {
                name: "Tim Beckham".to_owned(),
                role: PlayerType::Batter,
                team: "Chicago Pizzas".to_owned(),
            },
            Player {
                name: "Jacob deGrom".to_owned(),
                role: PlayerType::Pitcher,
                team: "Chicago Pizzas".to_owned(),
            },
            Player {
                name: "Carlos Rodón".to_owned(),
                role: PlayerType::Pitcher,
                team: "Chicago Pizzas".to_owned(),
            },
            Player {
                name: "Trey Mancini".to_owned(),
                role: PlayerType::Batter,
                team: "NY Hotdogs".to_owned(),
            },
            Player {
                name: "Anthony Rendon".to_owned(),
                role: PlayerType::Batter,
                team: "NY Hotdogs".to_owned(),
            },
            Player {
                name: "José Berríos".to_owned(),
                role: PlayerType::Pitcher,
                team: "NY Hotdogs".to_owned(),
            },
            Player {
                name: "Mike Clevinger".to_owned(),
                role: PlayerType::Pitcher,
                team: "NY Hotdogs".to_owned(),
            },
            Player {
                name: "Jonathan Villar".to_owned(),
                role: PlayerType::Batter,
                team: "Seattle Coffees".to_owned(),
            },
            Player {
                name: "Rhys Hoskins".to_owned(),
                role: PlayerType::Batter,
                team: "Seattle Coffees".to_owned(),
            },
            Player {
                name: "Kirby Yates".to_owned(),
                role: PlayerType::Pitcher,
                team: "Seattle Coffees".to_owned(),
            },
            Player {
                name: "Josh Hader".to_owned(),
                role: PlayerType::Pitcher,
                team: "Seattle Coffees".to_owned(),
            },
        ],
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn load_should_return_sample_roster_when_league_name_is_sample() {
        let roster = load(&"sample".to_owned()).unwrap();

        assert_eq!(16, roster.players.len());
    }
}
