use crate::utils;
use clap::ArgMatches;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::thread;
use std::time::Duration;

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
        true => get_players_from_file(filepath)?,
        false => get_players_from_sportradar(&config)?,
    };

    println!("raw_stats: {:#?}", raw_stats);

    Ok(())
}

fn get_players_from_sportradar(config: &Config) -> Result<Vec<Player>, Box<dyn Error>> {
    let game_ids = schedule::get_game_ids(config)?;
    // println!("game_ids: {:#?}", game_ids);

    let mut result: Vec<Player> = Vec::new();

    for id in game_ids.iter() {
        thread::sleep(Duration::from_millis(1050));
        let url = get_game_summary_url(config, id);
        println!("game summary api url: {}", url);

        let json = utils::get_json_res(&url)?;
        let mut players = get_players_from_string(json)?;
        result.append(&mut players);
    }

    Ok(result)
}

fn get_players_from_string(json: String) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut summary: Summary = serde_json::from_str(&json)?;
    let mut players: Vec<Player> = Vec::new();

    players.append(&mut summary.game.home.players);
    players.append(&mut summary.game.away.players);

    Ok(players)
}

fn get_game_summary_url(config: &Config, game_id: &String) -> String {
    format!(
        "https://api.sportradar.us/mlb-t6/games/{}/summary.json?api_key={}",
        game_id, config.api_key
    )
}

fn get_players_from_file(_filepath: &String) -> Result<Vec<Player>, Box<dyn Error>> {
    // FIXME
    Ok(vec![Default::default()])
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

#[derive(Deserialize, Debug)]
struct Summary {
    game: Game,
}

#[derive(Deserialize, Debug)]
struct Game {
    id: String,
    home: Team,
    away: Team,
}

#[derive(Deserialize, Debug)]
struct Team {
    abbr: String,
    players: Vec<Player>,
}

#[derive(Deserialize, Debug, Default)]
struct Player {
    preferred_name: String,
    last_name: String,
    status: String,
    position: String,
    primary_position: String,
    statistics: Stats,
}

#[derive(Deserialize, Debug, Default)]
struct Stats {
    hitting: Option<HittingStats>,
    pitching: Option<PitchingStats>,
}

#[derive(Deserialize, Debug, Default)]
struct HittingStats {
    overall: HittingOverallStats,
}

#[derive(Deserialize, Debug, Default)]
struct PitchingStats {
    overall: PitchingOverallStats,
}

#[derive(Deserialize, Debug, Default)]
struct HittingOverallStats {
    ab: u32,
    rbi: u32,
    onbase: HitterOnBaseStats,
    runs: HitterRunStats,
    outs: OutStats,
    steal: StealStats,
}

#[derive(Deserialize, Debug, Default)]
struct HitterOnBaseStats {
    s: u32,
    d: u32,
    t: u32,
    hr: u32,
    tb: u32,
    bb: u32,
    ibb: u32,
    hbp: u32,
    fc: u32,
    roe: u32,
    h: u32,
    cycle: u32,
}

#[derive(Deserialize, Debug, Default)]
struct HitterRunStats {
    total: u32,
}

#[derive(Deserialize, Debug, Default)]
struct OutStats {
    po: u32,
    fo: u32,
    fidp: u32,
    lo: u32,
    lidp: u32,
    go: u32,
    gidp: u32,
    klook: u32,
    kswing: u32,
    ktotal: u32,
    sacfly: u32,
    sachit: u32,
}

#[derive(Deserialize, Debug, Default)]
struct StealStats {
    caught: u32,
    stolen: u32,
    pickoff: u32,
}

#[derive(Deserialize, Debug, Default)]
struct PitchingOverallStats {
    lob: u32,
    pitch_count: u32,
    wp: u32,
    bk: u32,
    bf: u32,
    onbase: PitcherOnBaseStats,
    runs: PitcherRunStats,
    outs: OutStats,
    steal: StealStats,
    games: PitcherGameStats,
}

#[derive(Deserialize, Debug, Default)]
struct PitcherOnBaseStats {
    s: u32,
    d: u32,
    t: u32,
    hr: u32,
    tb: u32,
    bb: u32,
    ibb: u32,
    hbp: u32,
    fc: u32,
    roe: u32,
    h: u32,
}

#[derive(Deserialize, Debug, Default)]
struct PitcherRunStats {
    total: u32,
    unearned: u32,
    earned: u32,
}

#[derive(Deserialize, Debug, Default)]
struct PitcherGameStats {
    start: u32,
    play: u32,
    finish: u32,
    svo: u32,
    qstart: u32,
    shutout: u32,
    complete: u32,
    win: u32,
    loss: u32,
    save: u32,
    hold: u32,
    blown_save: u32,
    team_win: u32,
    team_loss: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_players_from_string_should_return_players() {
        use std::fs;
        let json = fs::read_to_string("testdata/summary.json").unwrap();
        let players = get_players_from_string(json).unwrap();
        assert_eq!(27, players.len());

        let hitter = players.iter().find(|p| p.last_name == "Trout").unwrap();
        // println!("hitter: {:#?}", hitter);
        assert_eq!(true, hitter.statistics.hitting.is_some());
        assert_eq!(true, hitter.statistics.pitching.is_none());

        let pitcher = players.iter().find(|p| p.last_name == "Cahill").unwrap();
        // println!("pitcher: {:#?}", pitcher);
        assert_eq!(true, pitcher.statistics.pitching.is_some());
        assert_eq!(true, pitcher.statistics.hitting.is_none());
    }
}
