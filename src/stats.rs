use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::Path;

mod schedule;

pub struct Config {
    date: String,
    league: String,
    api_key: String,
}

#[derive(Debug, Clone)]
struct ApiKeyNotFoundError;
impl fmt::Display for ApiKeyNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "both --api_key option and SPORTRADAR_API_KEY env are not set"
        )
    }
}
impl Error for ApiKeyNotFoundError {}

pub fn show(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let config = get_config(matches)?;

    let filepath = &format!("data/_stats/{}.json", config.date);

    // check stat data for given date exists.
    // if exists:
    //      use the file
    // else:
    //      request schedule for given date
    //      for each game in schedule:
    //          request summary for the game
    //          store all players stats to file
    let raw_stats = match Path::new(filepath).exists() {
        true => get_stats_from_file(filepath),
        false => get_stats_from_sportradar(&config),
    };

    Ok(())
}

// FIXME
struct PlayerStats;

fn get_stats_from_sportradar(config: &Config) -> Result<Vec<PlayerStats>, Box<dyn Error>> {
    // FIXME
    let game_ids = schedule::get_game_ids(config);
    println!("game_ids: {:#?}", game_ids);
    Ok(vec![PlayerStats])
}

fn get_stats_from_file(filepath: &String) -> Result<Vec<PlayerStats>, Box<dyn Error>> {
    // FIXME
    Ok(vec![PlayerStats])
}

fn get_config(matches: &ArgMatches) -> Result<Config, Box<dyn Error>> {
    let env_api_key = env::var("SPORTRADAR_API_KEY").unwrap_or("".to_owned());
    let api_key = matches.value_of("api_key").unwrap_or(env_api_key.as_str());
    if api_key == "" {
        return Err(Box::new(ApiKeyNotFoundError));
    }

    Ok(Config {
        date: matches.value_of("date").unwrap().to_owned(),
        league: matches.value_of("league").unwrap().to_owned(),
        api_key: api_key.to_owned(),
    })
}
