extern crate reqwest;

use crate::stats::Config;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Schedule {
    games: Vec<Game>,
}

#[derive(Deserialize, Debug)]
struct Game {
    id: String,
}

pub fn get_game_ids(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let url = get_schedule_url(config);
    println!("schedule api url: {}", url);

    let json = get_json_res(&url)?;
    Ok(get_games_ids_from_string(json)?)
}

fn get_schedule_url(config: &Config) -> String {
    let date = config.date.replace("-", "/");
    format!(
        "https://api.sportradar.us/mlb-t6/games/{}/schedule.json?api_key={}",
        date, config.api_key
    )
}

fn get_json_res(url: &String) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url)?.text()?)
}

fn get_games_ids_from_string(json: String) -> Result<Vec<String>, Box<dyn Error>> {
    let schedule: Schedule = serde_json::from_str(&json)?;
    Ok(schedule.games.into_iter().map(|g| g.id).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_schedule_url_should_return_api_url() {
        let config = Config {
            date: "2019-04-01".to_owned(),
            league: "some_league".to_owned(),
            api_key: "key".to_owned(),
        };
        assert_eq!(
            "https://api.sportradar.us/mlb-t6/games/2019/04/01/schedule.json?api_key=key",
            get_schedule_url(&config)
        );
    }

    #[test]
    fn get_games_ids_from_string_should_return_schedule() {
        use std::fs;
        let json = fs::read_to_string("temp/schedule.json").unwrap();
        let ids = get_games_ids_from_string(json).unwrap();
        assert_eq!(10, ids.len());
        assert_eq!("07d2922e-3f38-4dbe-a9ea-c96644b7dc10", ids.first().unwrap())
    }
}
