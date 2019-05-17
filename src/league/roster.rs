use crate::utils;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
pub struct Player<'a> {
    pub name: Cow<'a, str>,
    pub role: PlayerType,
    pub team: Rc<Cow<'a, str>>,
}
impl<'a> Player<'a> {
    fn new_batter<S>(name: S, team: S) -> Player<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Player {
            name: name.into(),
            role: PlayerType::Batter,
            team: Rc::new(team.into()),
        }
    }

    fn new_pitcher<S>(name: S, team: S) -> Player<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Player {
            name: name.into(),
            role: PlayerType::Pitcher,
            team: Rc::new(team.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Roster<'a> {
    pub players: Vec<Player<'a>>,
}

pub fn add(dir: &str) -> Result<Roster, Box<dyn Error>> {
    let filepath = &format!("{}/roster.json", dir);

    let roster = get_roster_from_stdin()?;

    fs::write(filepath, serde_json::to_string(&roster)?)?;
    println!("Saved weekly roster to {}.", filepath);

    Ok(roster)
}

fn get_roster_from_stdin<'a>() -> Result<Roster<'a>, Box<dyn Error>> {
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
        let team = Rc::new(Cow::Owned(team));
        let mut batters = get_players_stdin(PlayerType::Batter, num_batters, Rc::clone(&team));
        roster.players.append(&mut batters);

        let mut pitchers = get_players_stdin(PlayerType::Pitcher, num_pitchers, Rc::clone(&team));
        roster.players.append(&mut pitchers);
    }

    Ok(roster)
}

fn get_players_stdin<'a>(role: PlayerType, size: usize, team: Rc<Cow<'a, str>>) -> Vec<Player<'a>> {
    let mut players: Vec<Player> = Vec::new();
    let mut players_saved = 0;
    while players_saved < size {
        let label = format!(
            "Enter the name of {:?} {} for team {} (ex: John Doe) > ",
            role,
            players_saved + 1,
            team,
        );
        if let Ok(name) = get_string_stdin(label.as_str()) {
            players.push(Player {
                name: Cow::Owned(name),
                team: Rc::clone(&team),
                role,
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
        if !s.is_empty() {
            return Ok(s.to_string());
        }

        println!("Please input a non-empty string.");
    }
}

pub fn load(league_name: &str) -> Result<Roster, Box<dyn Error>> {
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

pub fn sample_roster<'a>() -> Roster<'a> {
    Roster {
        players: vec![
            Player::new_batter("Cody Bellinger", "LA Bulls"),
            Player::new_batter("Domingo Santana", "LA Bulls"),
            Player::new_pitcher("Blake Snell", "LA Bulls"),
            Player::new_pitcher("Max Scherzer", "LA Bulls"),
            Player::new_batter("Christian Yelich", "Chicago Pizzas"),
            Player::new_batter("Tim Beckham", "Chicago Pizzas"),
            Player::new_pitcher("Jacob deGrom", "Chicago Pizzas"),
            Player::new_pitcher("Carlos Rodón", "Chicago Pizzas"),
            Player::new_batter("Trey Mancini", "NY Hotdogs"),
            Player::new_batter("Anthony Rendon", "NY Hotdogs"),
            Player::new_pitcher("José Berríos", "NY Hotdogs"),
            Player::new_pitcher("Mike Clevinger", "NY Hotdogs"),
            Player::new_batter("Jonathan Villar", "Seattle Coffees"),
            Player::new_batter("Rhys Hoskins", "Seattle Coffees"),
            Player::new_pitcher("Kirby Yates", "Seattle Coffees"),
            Player::new_pitcher("Josh Hader", "Seattle Coffees"),
        ],
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn load_should_return_sample_roster_when_league_name_is_sample() {
        let league_name = "sample".to_string();
        let roster = load(&league_name).unwrap();

        assert_eq!(16, roster.players.len());
    }
}
